use windows::core::{Interface, Result};
use windows::Foundation::Numerics::Vector3;
use windows::{
    Foundation::Numerics::Vector2,
    UI::Composition::{Compositor, ContainerVisual, Visual},
};

use super::card::{Card, Face, Suit};
use super::composition_card::CompositionCard;
use super::shape_cache::ShapeCache;

pub struct Game {
    shape_cache: ShapeCache,
    compositor: Compositor,
    root: ContainerVisual,
}

impl Game {
    pub fn new(compositor: Compositor, size: Vector2, shape_cache: ShapeCache) -> Result<Self> {
        let root = compositor.CreateContainerVisual()?;
        root.SetRelativeSizeAdjustment(Vector2::new(1.0, 1.0))?;
        root.SetComment("Game Root")?;

        let debug_card = CompositionCard::new(Card::new(Face::King, Suit::Diamond), &shape_cache)?;
        let visual = debug_card.root()?;
        visual.SetScale(Vector3::new(3.0, 3.0, 1.0))?;
        root.Children()?.InsertAtTop(visual)?;
        std::mem::forget(debug_card);

        Ok(Self {
            shape_cache,
            compositor,
            root,
        })
    }

    pub fn root(&self) -> Result<Visual> {
        self.root.cast()
    }
}
