use std::{collections::HashMap};

use bindings::Windows::UI::Composition::{CompositionShape, Compositor};

use super::{card::{Card, Face, Suit}, svg::SvgCompositionShapes};

#[derive(PartialEq, Eq, Hash)]
pub enum ShapeType {
    Back,
    Empty
}

pub struct ShapeCache {
    geometry_cache: HashMap<Card, SvgCompositionShapes>,
    shape_cache: HashMap<ShapeType, CompositionShape>,
}

impl ShapeCache {
    pub fn new(compositor: &Compositor) -> windows::Result<Self> {
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

        Ok(Self {
            geometry_cache: HashMap::new(),
            shape_cache: HashMap::new(),
        })
    }
}