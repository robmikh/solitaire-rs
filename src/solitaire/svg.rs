use std::{collections::VecDeque, convert::TryInto, rc::Rc};
use windows::{Abi, Interface};

use bindings::Windows::{Foundation::Numerics::{Matrix3x2, Vector2, Vector3}, Graphics::IGeometrySource2D, UI::{Color, Colors, Composition::{CompositionBrush, CompositionContainerShape, CompositionGeometry, CompositionPath, CompositionViewBox, Compositor}}, Win32::{Foundation::PWSTR, Graphics::Direct2D::{D2D1_COLOR_F, D2D1_FILL_MODE_ALTERNATE, D2D1_SVG_ATTRIBUTE_POD_TYPE_COLOR, D2D1_SVG_ATTRIBUTE_POD_TYPE_FLOAT, D2D1_SVG_ATTRIBUTE_POD_TYPE_VIEWBOX, D2D1_SVG_ATTRIBUTE_STRING_TYPE_ID, D2D1_SVG_PAINT_TYPE_COLOR, D2D1_SVG_PAINT_TYPE_URI, D2D1_SVG_VIEWBOX, D2D_MATRIX_3X2_F, ID2D1SvgAttribute, ID2D1SvgDocument, ID2D1SvgElement, ID2D1SvgPaint, ID2D1SvgPathData}}};

use crate::util::{geometry_source::GeometrySource, try_cast::TryCast, wide_string::ToWide};

pub struct SvgCompositionShapes {
    pub view_box: Option<CompositionViewBox>,
    pub root_shape: CompositionContainerShape,
}

pub fn convert_svg_document_to_composition_shapes(compositor: &Compositor, document: &ID2D1SvgDocument) -> windows::Result<SvgCompositionShapes> {
    let root_visual = compositor.CreateShapeVisual()?;

    // Assumption: There is only one "svg" element and it is at the root.
    // Techincally this is incorrect, as the spec specifically states that
    // having multiple "svg" elements embeded in each other is possible.
    // https://www.w3.org/TR/SVG11/struct.html
    let mut root_element = None;
    unsafe { document.GetRoot(&mut root_element) };
    let root_element = root_element.unwrap();
    let tag = get_tag(&root_element)?;
    assert_eq!(&tag, "svg");

    let mut view_box = if unsafe { root_element.IsAttributeSpecified("viewBox".to_wide().as_pwstr(), std::ptr::null_mut()) }.as_bool() {
        let rect = get_rectangle_attribute(&root_element, "viewBox")?;
        let view_box = compositor.CreateViewBox()?;
        view_box.SetSize(Vector2::new(rect.width as f32, rect.height as f32))?;
        view_box.SetOffset(Vector2::new(rect.x as f32, rect.y as f32))?;
        Some(view_box)
    } else {
        None
    };

    let container = compositor.CreateContainerShape()?;
    let mut presentation_stack = VecDeque::new();
    presentation_stack.push_back(Presentation {
        fill: Some(Rc::new(Box::new(ColorBrushInfo::new(Colors::Black()?)))),
        stroke: Some(Rc::new(Box::new(ColorBrushInfo::new(Colors::Transparent()?)))),
        stroke_width: 1.0,
    });

    let mut child = None;
    unsafe { root_element.GetFirstChild(&mut child) };
    while child.is_some() {
        process_svg_element(&mut presentation_stack, &container, child.as_ref().unwrap())?;
        let next_child = unsafe { root_element.GetNextChild(child.as_ref().unwrap())? };
        if next_child.abi().is_null() {
            child = None;
        } else {
            child = Some(next_child);
        }
    }

    Ok(SvgCompositionShapes {
        view_box: view_box,
        root_shape: container,
    })
}

trait IBrushInfo {
    fn create_brush(&self, compositor: &Compositor) -> windows::Result<CompositionBrush>;
}

struct ColorBrushInfo {
    color: Color,
}

struct GradientStopInfo {
    offset: f32,
    color: Color,
}

struct LinearGradientBrushInfo {
    stops: Vec<GradientStopInfo>,
}

#[derive(Clone)]
struct Presentation {
    fill: Option<Rc<Box<dyn IBrushInfo>>>,
    stroke: Option<Rc<Box<dyn IBrushInfo>>>,
    stroke_width: f32,
}

impl ColorBrushInfo {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl LinearGradientBrushInfo {
    pub fn new(stops: Vec<GradientStopInfo>) -> Self {
        Self { stops }
    }
}

impl IBrushInfo for ColorBrushInfo {
    fn create_brush(&self, compositor: &Compositor) -> windows::Result<CompositionBrush> {
        compositor.CreateColorBrushWithColor(self.color)?.cast()
    }
}

impl IBrushInfo for LinearGradientBrushInfo {
    fn create_brush(&self, compositor: &Compositor) -> windows::Result<CompositionBrush> {
        let brush = compositor.CreateLinearGradientBrush()?;
        let color_stops = brush.ColorStops()?;
        for stop in &self.stops {
            let composition_stop = compositor.CreateColorGradientStopWithOffsetAndColor(stop.offset, stop.color)?;
            color_stops.Append(composition_stop)?;
        }
        brush.cast()
    }
}

fn get_tag(element: &ID2D1SvgElement) -> windows::Result<String> {
    let length = unsafe { element.GetTagNameLength() };
    let mut name = vec![0u16; (length + 1) as usize];
    unsafe { element.GetTagName(PWSTR(name.as_mut_ptr()), name.len() as u32)? };
    name.resize(length as usize, 0);
    let name = String::from_utf16_lossy(&name);
    Ok(name)
}

fn get_rectangle_attribute(element: &ID2D1SvgElement, attribute_name: &str) -> windows::Result<D2D1_SVG_VIEWBOX> {
    let mut rect = D2D1_SVG_VIEWBOX::default();
    unsafe {
        element.GetAttributeValue2(attribute_name.to_wide().as_pwstr(), D2D1_SVG_ATTRIBUTE_POD_TYPE_VIEWBOX, &mut rect as *mut _ as *mut _, std::mem::size_of::<D2D1_SVG_VIEWBOX>() as u32)?;
    }
    Ok(rect)
}

fn get_id_attribute(element: &ID2D1SvgElement, attribute_name: &str) -> windows::Result<String> {
    let attribute_name = attribute_name.to_wide();
    let attribute_length = unsafe { element.GetAttributeValueLength(attribute_name.as_pwstr(), D2D1_SVG_ATTRIBUTE_STRING_TYPE_ID)? };
    let mut data = vec![0u16; (attribute_length + 1) as usize];
    unsafe { element.GetAttributeValue(attribute_name.as_pwstr(), D2D1_SVG_ATTRIBUTE_STRING_TYPE_ID, PWSTR(data.as_mut_ptr()), data.len() as u32)? };
    data.resize(attribute_length as usize, 0);
    Ok(String::from_utf16_lossy(&data))
}

fn get_specified_attributes(element: &ID2D1SvgElement) -> windows::Result<Vec<String>> {
    let mut names = Vec::new();
    let count = unsafe { element.GetSpecifiedAttributeCount() };
    for i in 0..count {
        let mut length = 0;
        unsafe { element.GetSpecifiedAttributeNameLength(i, &mut length, &mut false.into())?; }
        let mut name = vec![0u16; (length + 1) as usize];
        unsafe { element.GetSpecifiedAttributeName(i, PWSTR(name.as_mut_ptr()), name.len() as u32, &mut false.into())?};
        name.resize(length as usize, 0);
        names.push(String::from_utf16_lossy(&name));
    }
    Ok(names)
}

fn get_float_attribute(element: &ID2D1SvgElement, attribute_name: &str) -> windows::Result<f32>{
    let mut value: f32 = 0.0;
    unsafe {
        element.GetAttributeValue2(attribute_name.to_wide().as_pwstr(), D2D1_SVG_ATTRIBUTE_POD_TYPE_FLOAT, &mut value as *mut _ as *mut _, std::mem::size_of::<f32>() as u32)?;
    };
    Ok(value)
}

fn get_color_attribute(element: &ID2D1SvgElement, attribute_name: &str) -> windows::Result<Color> {
    let mut value = D2D1_COLOR_F::default();
    unsafe { element.GetAttributeValue2(attribute_name.to_wide().as_pwstr(), D2D1_SVG_ATTRIBUTE_POD_TYPE_COLOR, &mut value as *mut _ as *mut _, std::mem::size_of::<D2D1_COLOR_F>() as u32)? };
    Ok(d2d_color_to_winrt_color(&value))
}

fn create_linear_gradient_brush_info(element: &ID2D1SvgElement) -> windows::Result<Rc<Box<dyn IBrushInfo>>> {
    let mut stops = Vec::new();
    if unsafe { element.HasChildren().as_bool() } {
        let mut child = None;
        unsafe { element.GetFirstChild(&mut child)};
        while child.is_some() {
            let current = child.as_ref().unwrap();
            if unsafe { current.IsTextContent().as_bool() } {
                let offset = get_float_attribute(current, "offset")?;
                let color = get_color_attribute(current, "stop-color")?;
                let stop = GradientStopInfo { offset, color };
                stops.push(stop);
            }
        }
    }
    Ok(Rc::new(Box::new(LinearGradientBrushInfo::new(stops))))
}   

fn create_brush_info_from_id(document: &ID2D1SvgDocument, mut id: Vec<u16>) -> windows::Result<Option<Rc<Box<dyn IBrushInfo>>>> {
    let reference = unsafe { document.FindElementById(PWSTR(id.as_mut_ptr()))? };
    let tag = get_tag(&reference)?;
    if tag == "linearGradient" {
        Ok(Some(create_linear_gradient_brush_info(&reference)?))
    } else {
        Ok(None)
    }
}

fn get_brush_info(element: &ID2D1SvgElement, attribute_name: &str) -> windows::Result<Option<Rc<Box<dyn IBrushInfo>>>> {
    let attribute: ID2D1SvgAttribute = unsafe { element.GetAttributeValue3(attribute_name.to_wide().as_pwstr())? };
    if let Some(paint_attribute) = attribute.try_cast::<ID2D1SvgPaint>()? {
        let paint_type = unsafe { paint_attribute.GetPaintType() };
        match paint_type {
            D2D1_SVG_PAINT_TYPE_COLOR => {
                let mut color = D2D1_COLOR_F::default();
                unsafe { paint_attribute.GetColor(&mut color) };
                return Ok(Some(Rc::new(Box::new(ColorBrushInfo::new(d2d_color_to_winrt_color(&color))))));
            }
            D2D1_SVG_PAINT_TYPE_URI => {
                let length = unsafe { paint_attribute.GetIdLength() };
                let mut id = vec![0u16; (length + 1) as usize];
                unsafe {paint_attribute.GetId(PWSTR(id.as_mut_ptr()), length)? };
                id.resize(length as usize, 0);
                let mut document = None;
                unsafe { element.GetDocument(&mut document)};
                let document = document.unwrap();
                return create_brush_info_from_id(&document, id);
            }
            _ => {}
        }
    }
    Ok(None)
}

fn d2d_color_to_winrt_color(color: &D2D1_COLOR_F) -> Color {
    Color { A: (color.a * 255.0) as u8, R: (color.r * 255.0) as u8, G: (color.g * 255.0) as u8, B: (color.b * 255.0) as u8 }
}

fn get_transform_attribute(element: &ID2D1SvgElement, attribute_name: &str) -> windows::Result<Matrix3x2> {
    let mut value = Matrix3x2::default();
    unsafe { element.GetAttributeValue2(attribute_name.to_wide().as_pwstr(), D2D1_SVG_ATTRIBUTE_POD_TYPE_COLOR, &mut value as *mut _ as *mut _, std::mem::size_of::<D2D_MATRIX_3X2_F>() as u32)? };
    Ok(value)
}

fn create_brush_from_brush_info(compositor: &Compositor, brush_info: &Option<Rc<Box<dyn IBrushInfo>>>) -> windows::Result<Option<CompositionBrush>> {
    if let Some(info) = brush_info {
        Ok(Some(info.create_brush(compositor)?))
    } else {
        Ok(None)
    }
}

fn process_svg_element(presentation_stack: &mut VecDeque<Presentation>, parent_shape: &CompositionContainerShape, element: &ID2D1SvgElement) -> windows::Result<()> {
    let mut current = element;
    if !unsafe { current.IsTextContent().as_bool() } {
        let compositor = parent_shape.Compositor()?;
        let current_shape = compositor.CreateContainerShape()?;
        parent_shape.Shapes()?.Append(&current_shape)?;

        let current_presentation = &presentation_stack[0];
        let mut presentation = current_presentation.clone();

        // Record the id for debugging
        if unsafe { current.IsAttributeSpecified("id".to_wide().as_pwstr(), std::ptr::null_mut()).as_bool() } {
            let id = get_id_attribute(&element, "id")?;
            current_shape.SetComment(id)?;
        }

        let mut offset = Vector2::new(0.0, 0.0);

        // General attributes
        let attribute_names = get_specified_attributes(&current)?;
        for attribute_name in &attribute_names {
            match attribute_name.as_str() {
                "x" => {
                    let value = get_float_attribute(current, attribute_name)?;
                    offset.X = value;
                }
                "y" => {
                    let value = get_float_attribute(current, attribute_name)?;
                    offset.Y = value;
                }
                "fill" => {
                    presentation.fill = get_brush_info(current, attribute_name)?;
                }
                "stroke" => {
                    presentation.stroke = get_brush_info(current, attribute_name)?;
                }
                "stroke-width" => {
                    presentation.stroke_width = get_float_attribute(current, attribute_name)?;
                }
                "transform" => {
                    let transform = get_transform_attribute(current, attribute_name)?;
                    current_shape.SetTransformMatrix(transform)?;
                }
                _ => {}
            }
        }
        current_shape.SetOffset(offset)?;

        // Special cases
        let tag = get_tag(current)?;
        match tag.as_str() {
            "defs" => {
                // TODO: Keep track of children, prevent them from rendering
                // For now, let's just disconnect ourselves from the tree. A bit nasty.
                parent_shape.Shapes()?.RemoveAtEnd()?;
            }
            "use" => {
                let href = get_id_attribute(current, "xlink:href")?;
                let mut document = None;
                unsafe { current.GetDocument(&mut document) };
                let document = document.unwrap();
                let reference = unsafe { document.FindElementById(href.as_str().to_wide().as_pwstr())? };
                process_svg_element(presentation_stack, &current_shape, &reference)?;
            }
            "circle" => {
                let center_x = get_float_attribute(current, "cx")?;
                let center_y = get_float_attribute(current, "cy")?;
                let radius = get_float_attribute(current, "r")?;

                let geometry = compositor.CreateEllipseGeometry()?;
                geometry.SetCenter(Vector2::new(center_x, center_y))?;
                geometry.SetRadius(Vector2::new(radius, radius))?;
                let sprite_shape = compositor.CreateSpriteShape()?;

                sprite_shape.SetFillBrush(create_brush_from_brush_info(&compositor, &presentation.fill)?)?;
                sprite_shape.SetStrokeBrush(create_brush_from_brush_info(&compositor, &presentation.stroke)?)?;
                sprite_shape.SetStrokeThickness(presentation.stroke_width)?;

                current_shape.Shapes()?.Append(sprite_shape)?;
            }
            "rect" => {
                let x = get_float_attribute(current, "x")?;
                let y = get_float_attribute(current, "y")?;
                let width = get_float_attribute(current, "width")?;
                let height = get_float_attribute(current, "height")?;

                let geometry = compositor.CreateRectangleGeometry()?;
                geometry.SetOffset(Vector2::new(x, y))?;
                geometry.SetSize(Vector2::new(width, height))?;
                let sprite_shape = compositor.CreateSpriteShape()?;

                sprite_shape.SetFillBrush(create_brush_from_brush_info(&compositor, &presentation.fill)?)?;
                sprite_shape.SetStrokeBrush(create_brush_from_brush_info(&compositor, &presentation.stroke)?)?;
                sprite_shape.SetStrokeThickness(presentation.stroke_width)?;

                current_shape.Shapes()?.Append(sprite_shape)?;
            }
            "g" => {}
            "path" => {
                let attribute: ID2D1SvgAttribute = unsafe { element.GetAttributeValue3("d".to_wide().as_pwstr())? };
                let path_attribute: ID2D1SvgPathData = attribute.cast()?;
                let d2d_geometry = unsafe {path_attribute.CreatePathGeometry(D2D1_FILL_MODE_ALTERNATE)? };
                let geometry_source: IGeometrySource2D = GeometrySource::new(d2d_geometry.cast()?).into();
                let composition_path = CompositionPath::Create(geometry_source)?;
                let path_geometry: CompositionGeometry = compositor.CreatePathGeometryWithPath(composition_path)?.cast()?;
                let sprite_shape = compositor.CreateSpriteShapeWithGeometry(path_geometry)?;

                sprite_shape.SetFillBrush(create_brush_from_brush_info(&compositor, &presentation.fill)?)?;
                sprite_shape.SetStrokeBrush(create_brush_from_brush_info(&compositor, &presentation.stroke)?)?;
                sprite_shape.SetStrokeThickness(presentation.stroke_width)?;

                current_shape.Shapes()?.Append(sprite_shape)?;
            }
            // Currently these are handled elsewhere.
            "stop" => {}
            "linearGradient" => {}
            "radialGradient" => {}
            _ => {
                println!("Unknown tag: {}", &tag);
            }
        }

        presentation_stack.push_back(presentation);

        // Process children
        if unsafe { current.HasChildren().as_bool() } {
            let mut child = None;
            unsafe { current.GetFirstChild(&mut child) };
            while child.is_some() {
                process_svg_element(presentation_stack, &current_shape, child.as_ref().unwrap())?;
                let next_child = unsafe { current.GetNextChild(child.as_ref().unwrap())? };
                if next_child.abi().is_null() {
                    child = None;
                } else {
                    child = Some(next_child);
                }
            }
        }

        presentation_stack.pop_back();
    }
    Ok(())
}