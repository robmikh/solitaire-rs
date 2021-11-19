use std::collections::HashMap;

use windows::core::{Interface, Result};
use windows::{
    Foundation::Numerics::Vector2,
    Win32::{
        Graphics::Direct2D::{
            Common::D2D_SIZE_F, ID2D1DeviceContext5, D2D1_DEVICE_CONTEXT_OPTIONS_NONE,
        },
        System::Com::IStream,
    },
    UI::{
        Colors,
        Composition::{CompositionBrush, CompositionShape, CompositionSpriteShape, Compositor},
    },
};

use crate::util::{
    d2d::{create_d2d_device, create_d2d_factory},
    d3d::create_d3d_device,
    streams::ReadOnlyCursorToStreamWrapper,
};

use super::{
    assets::get_asset_data,
    card::{Card, Face, Suit},
    composition_card::CompositionCard,
    svg::{convert_svg_document_to_composition_shapes, SvgCompositionShapes},
};

#[derive(PartialEq, Eq, Hash)]
pub enum ShapeType {
    Back,
    Empty,
}

pub struct ShapeCache {
    compositor: Compositor,
    geometry_cache: HashMap<Card, SvgCompositionShapes>,
    shape_cache: HashMap<ShapeType, CompositionShape>,
    text_height: f32,
}

impl ShapeCache {
    pub fn new(compositor: &Compositor) -> Result<Self> {
        let cards = {
            let mut cards = Vec::new();
            for i in 0..Face::King as i32 {
                let face: Face = unsafe { std::mem::transmute(i + 1) };
                for j in 0..(Suit::Club as i32 + 1) {
                    let suit: Suit = unsafe { std::mem::transmute(j) };
                    let card = Card { face, suit };
                    cards.push(card);
                }
            }
            cards
        };

        let d3d_device = create_d3d_device()?;
        let d2d_factory = create_d2d_factory()?;
        let d2d_device = create_d2d_device(&d2d_factory, &d3d_device)?;
        let d2d_context: ID2D1DeviceContext5 = unsafe {
            d2d_device
                .CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE)?
                .cast()?
        };

        let mut geometry_cache = HashMap::new();
        let height = 34.0; // TODO: I guess this should be hardcoded now, get the right number later
        for card in cards {
            let asset_name = get_svg_file_name(&card);
            let data = get_asset_data(&asset_name).unwrap();
            let stream: IStream = ReadOnlyCursorToStreamWrapper::new(data).into();
            let viewport = D2D_SIZE_F {
                width: 1.0,
                height: 1.0,
            };
            let document = unsafe { d2d_context.CreateSvgDocument(&stream, &viewport)? };
            let shape_info = convert_svg_document_to_composition_shapes(compositor, &document)?;
            geometry_cache.insert(card, shape_info);
        }

        let mut shape_cache = HashMap::new();
        {
            let shape_container = compositor.CreateContainerShape()?;
            let background_base_color = Colors::Blue()?;

            let rect_shape = build_rounded_rect_shape(
                compositor,
                &CompositionCard::CardSize,
                &CompositionCard::CornerRadius,
                0.5,
                Some(
                    compositor
                        .CreateColorBrushWithColor(Colors::White()?)?
                        .cast()?,
                ),
                Some(
                    compositor
                        .CreateColorBrushWithColor(background_base_color)?
                        .cast()?,
                ),
            )?;
            shape_container.Shapes()?.Append(rect_shape)?;

            let inner_offset = Vector2::new(12.0, 12.0);
            let inner_rect_shape = build_rounded_rect_shape(
                compositor,
                &(CompositionCard::CardSize - &inner_offset),
                &CompositionCard::CornerRadius,
                5.0,
                Some(
                    compositor
                        .CreateColorBrushWithColor(Colors::White()?)?
                        .cast()?,
                ),
                Some(
                    compositor
                        .CreateColorBrushWithColor(background_base_color)?
                        .cast()?,
                ),
            )?;
            inner_rect_shape.SetOffset(inner_rect_shape.Offset()? + (inner_offset / 2.0))?;
            shape_container.Shapes()?.Append(inner_rect_shape)?;

            shape_cache.insert(ShapeType::Back, shape_container.cast::<CompositionShape>()?);
        }
        {
            let shape_container = compositor.CreateContainerShape()?;

            let rect_shape = build_rounded_rect_shape(
                compositor,
                &CompositionCard::CardSize,
                &CompositionCard::CornerRadius,
                5.0,
                Some(
                    compositor
                        .CreateColorBrushWithColor(Colors::Gray()?)?
                        .cast()?,
                ),
                None,
            )?;
            shape_container.Shapes()?.Append(rect_shape)?;

            let inner_size = CompositionCard::CardSize / 2.0;
            let inner_rounded_rect_geometry = compositor.CreateRoundedRectangleGeometry()?;
            inner_rounded_rect_geometry.SetCornerRadius(CompositionCard::CornerRadius)?;
            inner_rounded_rect_geometry.SetSize(inner_size)?;
            let inner_rect_shape =
                compositor.CreateSpriteShapeWithGeometry(inner_rounded_rect_geometry)?;
            inner_rect_shape
                .SetFillBrush(compositor.CreateColorBrushWithColor(Colors::Gray()?)?)?;
            inner_rect_shape.SetStrokeThickness(5.0)?;
            inner_rect_shape.SetOffset((CompositionCard::CardSize - inner_size) / 2.0)?;
            shape_container.Shapes()?.Append(inner_rect_shape)?;

            shape_cache.insert(ShapeType::Back, shape_container.cast::<CompositionShape>()?);
        }

        Ok(Self {
            compositor: compositor.clone(),
            geometry_cache,
            shape_cache,
            text_height: height,
        })
    }

    pub fn compositor(&self) -> &Compositor {
        &self.compositor
    }

    pub fn get_card_face(&self, key: &Card) -> &SvgCompositionShapes {
        self.geometry_cache.get(key).unwrap()
    }

    pub fn get_shape(&self, shape_type: &ShapeType) -> &CompositionShape {
        self.shape_cache.get(shape_type).unwrap()
    }

    pub fn text_height(&self) -> f32 {
        self.text_height
    }
}

pub fn get_svg_file_name(card: &Card) -> String {
    let mut file_name = String::new();
    match card.face {
        Face::Ace => file_name.push_str("ace"),
        Face::Two => file_name.push_str("2"),
        Face::Three => file_name.push_str("3"),
        Face::Four => file_name.push_str("4"),
        Face::Five => file_name.push_str("5"),
        Face::Six => file_name.push_str("6"),
        Face::Seven => file_name.push_str("7"),
        Face::Eight => file_name.push_str("8"),
        Face::Nine => file_name.push_str("9"),
        Face::Ten => file_name.push_str("10"),
        Face::Jack => file_name.push_str("jack"),
        Face::Queen => file_name.push_str("queen"),
        Face::King => file_name.push_str("king"),
    }
    file_name.push_str("_of_");
    match card.suit {
        Suit::Diamond => file_name.push_str("diamonds"),
        Suit::Spade => file_name.push_str("spades"),
        Suit::Heart => file_name.push_str("hearts"),
        Suit::Club => file_name.push_str("clubs"),
    }
    file_name
}

fn build_rounded_rect_shape(
    compositor: &Compositor,
    size: &Vector2,
    corner_radius: &Vector2,
    stroke_thickness: f32,
    stroke_brush: Option<CompositionBrush>,
    fill_brush: Option<CompositionBrush>,
) -> Result<CompositionSpriteShape> {
    let stroke_added_size = Vector2::new(stroke_thickness, stroke_thickness);
    let stroke_offset = stroke_added_size / 2.0;
    let rounded_rect_geometry = compositor.CreateRoundedRectangleGeometry()?;
    rounded_rect_geometry.SetCornerRadius(corner_radius)?;
    rounded_rect_geometry.SetSize(size)?;
    let rect_shape = compositor.CreateSpriteShapeWithGeometry(rounded_rect_geometry)?;
    rect_shape.SetStrokeBrush(stroke_brush)?;
    rect_shape.SetFillBrush(fill_brush)?;
    rect_shape.SetStrokeThickness(stroke_thickness)?;
    rect_shape.SetOffset(stroke_offset)?;
    Ok(rect_shape)
}
