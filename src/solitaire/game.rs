use bindings::Windows::{Foundation::Numerics::Vector2, UI::Composition::{Compositor, ContainerVisual, Visual}};
use windows::Interface;

use super::shape_cache::ShapeCache;

pub struct Game {
    shape_cache: ShapeCache,
    compositor: Compositor,
    root: ContainerVisual,
}

impl Game {
    pub fn new(compositor: Compositor, size: Vector2, shape_cache: ShapeCache) -> windows::Result<Self> {
        let root = compositor.CreateContainerVisual()?;
        root.SetRelativeSizeAdjustment(Vector2::new(1.0, 1.0))?;
        root.SetComment("Game Root")?;

        Ok(Self{
            shape_cache,
            compositor,
            root,
        })
    }

    pub fn root(&self) -> windows::Result<Visual> {
        self.root.cast()
    }
}