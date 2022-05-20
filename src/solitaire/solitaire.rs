use windows::{
    core::Result,
    Foundation::Numerics::{Vector2, Vector3},
    UI::{
        Color,
        Composition::{ContainerVisual, SpriteVisual},
    },
};

use super::{game::Game, shape_cache::ShapeCache};

pub struct Solitaire {
    last_parent_size: Vector2,
    game: Game,
    root: ContainerVisual,
    background: SpriteVisual,
    content: ContainerVisual,
}

impl Solitaire {
    pub fn new(parent_visual: &ContainerVisual, parent_size: Vector2) -> Result<Self> {
        let compositor = parent_visual.Compositor()?;
        let shape_cache = ShapeCache::new(&compositor)?;

        let root = compositor.CreateContainerVisual()?;
        root.SetRelativeSizeAdjustment(Vector2::new(1.0, 1.0))?;
        root.SetComment("Application Root")?;
        parent_visual.Children()?.InsertAtTop(&root)?;
        let root_children = root.Children()?;

        let background = compositor.CreateSpriteVisual()?;
        background.SetAnchorPoint(Vector2::new(0.5, 0.5))?;
        background.SetRelativeOffsetAdjustment(Vector3::new(0.5, 0.5, 0.0))?;
        let diameter = compute_diameter(&parent_size) * 2.0;
        background.SetSize(Vector2::new(diameter, diameter))?;
        let background_brush = compositor.CreateRadialGradientBrush()?;
        let color_stops = background_brush.ColorStops()?;
        color_stops.Append(compositor.CreateColorGradientStopWithOffsetAndColor(
            0.0,
            Color {
                A: 255,
                R: 14,
                G: 144,
                B: 58,
            },
        )?)?;
        color_stops.Append(compositor.CreateColorGradientStopWithOffsetAndColor(
            1.0,
            Color {
                A: 255,
                R: 7,
                G: 69,
                B: 32,
            },
        )?)?;
        background.SetBrush(background_brush)?;
        root_children.InsertAtBottom(&background)?;

        let content = compositor.CreateContainerVisual()?;
        content.SetSize(Vector2::new(1327.0, 1111.0))?;
        content.SetAnchorPoint(Vector2::new(0.5, 0.5))?;
        content.SetRelativeOffsetAdjustment(Vector3::new(0.5, 0.5, 0.0))?;
        let scale = compute_scale_factor(&parent_size, &content.Size()?);
        content.SetScale(Vector3::new(scale, scale, 1.0))?;
        root_children.InsertAtTop(&content)?;

        let debug_card = super::composition_card::CompositionCard::new(
            super::card::Card::new(super::card::Face::King, super::card::Suit::Diamond),
            &shape_cache,
        )?;
        let visual = debug_card.root()?;
        visual.SetScale(Vector3::new(3.0, 3.0, 1.0))?;
        root_children.InsertAtTop(visual)?;
        std::mem::forget(debug_card);

        let size = content.Size()?;
        let game = Game::new(compositor, size, shape_cache)?;
        content.Children()?.InsertAtTop(game.root()?)?;
        Ok(Self {
            last_parent_size: parent_size,
            game,
            root,
            background,
            content,
        })
    }
}

fn compute_diameter(parent_size: &Vector2) -> f32 {
    (parent_size.X.powf(2.0) + parent_size.Y.powf(2.0)).sqrt()
}

fn compute_scale_factor(parent_size: &Vector2, content_size: &Vector2) -> f32 {
    let parent_ratio = parent_size.X / parent_size.Y;
    let content_ratio = content_size.X / content_size.Y;

    if parent_ratio > content_ratio {
        parent_size.Y / content_size.Y
    } else {
        parent_size.X / content_size.X
    }
}
