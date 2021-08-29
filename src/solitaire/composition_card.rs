use std::path::Components;

use bindings::Windows::Foundation::Numerics::Vector2;

pub struct CompositionCard {

}

impl CompositionCard {
    pub const CardSize: Vector2 = Vector2{ X: 167.0, Y: 243.0 };
    pub const CornerRadius: Vector2 = Vector2{ X: 9.5, Y: 9.5 };
    
}