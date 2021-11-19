use windows::{
    core::{Interface, Result},
    Foundation::Numerics::{Vector2, Vector3},
    UI::Composition::{CompositionBackfaceVisibility, ContainerVisual, ShapeVisual, Visual},
};

use super::{
    card::Card,
    shape_cache::{ShapeCache, ShapeType},
};

pub struct CompositionCard {
    root: ContainerVisual,
    sides_root: ContainerVisual,
    front: ShapeVisual,
    back: ShapeVisual,
    card: Card,
    is_face_up: bool,
}

impl CompositionCard {
    pub const CardSize: Vector2 = Vector2 { X: 167.0, Y: 243.0 };
    pub const CornerRadius: Vector2 = Vector2 { X: 9.5, Y: 9.5 };

    pub fn new(card: Card, shape_cache: &ShapeCache) -> Result<Self> {
        let compositor = shape_cache.compositor();

        let root = compositor.CreateContainerVisual()?;
        root.SetSize(Self::CardSize)?;
        root.SetComment("Card Root")?;

        let sides_root = compositor.CreateContainerVisual()?;
        sides_root.SetRelativeSizeAdjustment(Vector2::new(1.0, 1.0))?;
        sides_root.SetRotationAxis(Vector3::new(0.0, 1.0, 0.0))?;
        sides_root.SetCenterPoint(Vector3::new(
            Self::CardSize.X / 2.0,
            Self::CardSize.Y / 2.0,
            0.0,
        ))?;
        sides_root.SetComment("Card Sides")?;
        root.Children()?.InsertAtTop(&sides_root)?;
        let front = build_card_front(shape_cache, &card)?;
        sides_root.Children()?.InsertAtTop(&front)?;
        let back = build_card_back(shape_cache)?;
        sides_root.Children()?.InsertAtTop(&back)?;

        Ok(Self {
            root,
            sides_root,
            front,
            back,
            card,
            is_face_up: false,
        })
    }

    pub fn root(&self) -> Result<Visual> {
        self.root.cast()
    }
}

fn build_card_back(shape_cache: &ShapeCache) -> Result<ShapeVisual> {
    let compositor = shape_cache.compositor();
    let shape_visual = compositor.CreateShapeVisual()?;
    shape_visual
        .Shapes()?
        .Append(shape_cache.get_shape(&ShapeType::Back))?;
    shape_visual.SetSize(CompositionCard::CardSize)?;
    shape_visual.SetBackfaceVisibility(CompositionBackfaceVisibility::Hidden)?;
    shape_visual.SetRotationAxis(Vector3::new(0.0, 1.0, 0.0))?;
    shape_visual.SetRotationAngleInDegrees(180.0)?;
    shape_visual.SetCenterPoint(Vector3::new(
        CompositionCard::CardSize.X / 2.0,
        CompositionCard::CardSize.Y / 2.0,
        0.0,
    ))?;
    shape_visual.SetComment("Card Back")?;
    Ok(shape_visual)
}

fn build_card_front(shape_cache: &ShapeCache, card: &Card) -> Result<ShapeVisual> {
    let compositor = shape_cache.compositor();
    let shape_visual = compositor.CreateShapeVisual()?;
    let shape_container = compositor.CreateContainerShape()?;
    shape_visual.Shapes()?.Append(&shape_container)?;
    shape_visual.SetSize(CompositionCard::CardSize)?;
    shape_visual.SetBackfaceVisibility(CompositionBackfaceVisibility::Hidden)?;

    let shape_info = shape_cache.get_card_face(card);
    shape_container.Shapes()?.Append(&shape_info.root_shape)?;
    shape_visual.SetViewBox(&shape_info.view_box)?;

    shape_visual.SetComment(card.to_string())?;

    Ok(shape_visual)
}
