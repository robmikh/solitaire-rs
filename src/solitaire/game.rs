use windows::UI::Composition::VisualCollection;
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
    compositor: Compositor,
    root: ContainerVisual,

    board_layer: ContainerVisual,
    foundation_visual: ContainerVisual,
    deck_visual: ContainerVisual,
    waste_visual: ContainerVisual,
    play_area_visual: ContainerVisual,
    selected_layer: ContainerVisual,
    visuals: VisualCollection,

    selected_visual: Option<Visual>,
    // selected_cards
    // selected_item_containers
    // last_operation
    // last_pile
    // last_hit_test
    offset: Vector2,

    is_deck_animation_running: bool,
    layout_info: LayoutInformation,

    shape_cache: ShapeCache,
    // pack
    // stacks
    // zone_rects
    // deck
    // waste
    // foundations
}

struct LayoutInformation {
    pub card_stack_vertical_offset: f32,
    pub waste_horizontal_offset: f32,
}

impl Default for LayoutInformation {
    fn default() -> Self {
        Self { 
            card_stack_vertical_offset: 47.88, 
            waste_horizontal_offset: 65.0 
        }
    }
}

impl Game {
    pub fn new(compositor: Compositor, size: Vector2, shape_cache: ShapeCache) -> Result<Self> {
        // Base visual tree
        let root = compositor.CreateContainerVisual()?;
        root.SetRelativeSizeAdjustment(Vector2::new(1.0, 1.0))?;
        root.SetComment("Game Root")?;
        let root_children = root.Children()?;

        let board_layer = compositor.CreateContainerVisual()?;
        board_layer.SetRelativeSizeAdjustment(Vector2::new(1.0, 1.0))?;
        board_layer.SetComment("Board Layer")?;
        root_children.InsertAtTop(&board_layer)?;

        let selected_layer = compositor.CreateContainerVisual()?;
        selected_layer.SetRelativeSizeAdjustment(Vector2::new(1.0, 1.0))?;
        selected_layer.SetComment("Selection Layer")?;
        root_children.InsertAtTop(&selected_layer)?;

        let visuals = board_layer.Children()?;

        // Get layout info
        let text_height = shape_cache.text_height();
        let layout_info = LayoutInformation {
            card_stack_vertical_offset: text_height,
            .. Default::default()
        };
        let card_size = CompositionCard::CardSize;

        // Play area
        let play_area_offset_y = card_size.Y + 25.0;
        let play_area_visual = compositor.CreateContainerVisual()?;
        play_area_visual.SetOffset(Vector3::new(0.0, play_area_offset_y, 0.0))?;
        play_area_visual.SetSize(Vector2::new(0.0, -1.0 * play_area_offset_y))?;
        play_area_visual.SetRelativeSizeAdjustment(Vector2::new(1.0, 1.0))?;
        play_area_visual.SetComment("Play Area Root")?;
        visuals.InsertAtTop(&play_area_visual)?;
        // zone rects insert

        // Deck
        let deck_visual = compositor.CreateContainerVisual()?;
        deck_visual.SetSize(&card_size)?;
        deck_visual.SetOffset(Vector3::new(card_size.X + 25.0, 0.0, 0.0))?;
        deck_visual.SetComment("Deck Area Root")?;
        visuals.InsertAtTop(&deck_visual)?;
        // zone rects insert

        // Waste
        let waste_visual = compositor.CreateContainerVisual()?;
        waste_visual.SetSize(Vector2::new((2.0 * layout_info.waste_horizontal_offset) + card_size.X, card_size.Y))?;
        waste_visual.SetOffset(Vector3::new(card_size.X + 25.0, 0.0, 0.0))?;
        waste_visual.SetComment("Waste Area Root")?;
        visuals.InsertAtTop(&waste_visual)?;
        // zone rects insert

        // Foundation
        let foundation_visual = compositor.CreateContainerVisual()?;
        foundation_visual.SetSize(Vector2::new(4.0 * card_size.X + 3.0 * 15.0, card_size.Y))?;
        foundation_visual.SetAnchorPoint(Vector2::new(1.0, 0.0))?;
        foundation_visual.SetRelativeOffsetAdjustment(Vector3::new(1.0, 0.0, 0.0))?;
        foundation_visual.SetComment("Foundations Root")?;
        visuals.InsertAtTop(&foundation_visual)?;
        // zone rects insert

        // new game

        Ok(Self {
            compositor,
            root,

            board_layer,
            foundation_visual,
            deck_visual,
            waste_visual,
            play_area_visual,
            selected_layer,
            visuals,

            selected_visual: None,
            offset: Vector2::new(0.0, 0.0),

            is_deck_animation_running: false,
            layout_info,

            shape_cache,
        })
    }

    pub fn root(&self) -> Result<Visual> {
        self.root.cast()
    }
}
