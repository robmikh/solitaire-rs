static TWO_OF_CLUBS: &[u8] = include_bytes!("../../assets/card_faces/2_of_clubs.svg");
static TWO_OF_DIAMONDS: &[u8] = include_bytes!("../../assets/card_faces/2_of_diamonds.svg");
static TWO_OF_HEARTS: &[u8] = include_bytes!("../../assets/card_faces/2_of_hearts.svg");
static TWO_OF_SPADES: &[u8] = include_bytes!("../../assets/card_faces/2_of_spades.svg");

static THREE_OF_CLUBS: &[u8] = include_bytes!("../../assets/card_faces/3_of_clubs.svg");
static THREE_OF_DIAMONDS: &[u8] = include_bytes!("../../assets/card_faces/3_of_diamonds.svg");
static THREE_OF_HEARTS: &[u8] = include_bytes!("../../assets/card_faces/3_of_hearts.svg");
static THREE_OF_SPADES: &[u8] = include_bytes!("../../assets/card_faces/3_of_spades.svg");

static FOUR_OF_CLUBS: &[u8] = include_bytes!("../../assets/card_faces/4_of_clubs.svg");
static FOUR_OF_DIAMONDS: &[u8] = include_bytes!("../../assets/card_faces/4_of_diamonds.svg");
static FOUR_OF_HEARTS: &[u8] = include_bytes!("../../assets/card_faces/4_of_hearts.svg");
static FOUR_OF_SPADES: &[u8] = include_bytes!("../../assets/card_faces/4_of_spades.svg");

static FIVE_OF_CLUBS: &[u8] = include_bytes!("../../assets/card_faces/5_of_clubs.svg");
static FIVE_OF_DIAMONDS: &[u8] = include_bytes!("../../assets/card_faces/5_of_diamonds.svg");
static FIVE_OF_HEARTS: &[u8] = include_bytes!("../../assets/card_faces/5_of_hearts.svg");
static FIVE_OF_SPADES: &[u8] = include_bytes!("../../assets/card_faces/5_of_spades.svg");

static SIX_OF_CLUBS: &[u8] = include_bytes!("../../assets/card_faces/6_of_clubs.svg");
static SIX_OF_DIAMONDS: &[u8] = include_bytes!("../../assets/card_faces/6_of_diamonds.svg");
static SIX_OF_HEARTS: &[u8] = include_bytes!("../../assets/card_faces/6_of_hearts.svg");
static SIX_OF_SPADES: &[u8] = include_bytes!("../../assets/card_faces/6_of_spades.svg");

static SEVEN_OF_CLUBS: &[u8] = include_bytes!("../../assets/card_faces/7_of_clubs.svg");
static SEVEN_OF_DIAMONDS: &[u8] = include_bytes!("../../assets/card_faces/7_of_diamonds.svg");
static SEVEN_OF_HEARTS: &[u8] = include_bytes!("../../assets/card_faces/7_of_hearts.svg");
static SEVEN_OF_SPADES: &[u8] = include_bytes!("../../assets/card_faces/7_of_spades.svg");

static EIGHT_OF_CLUBS: &[u8] = include_bytes!("../../assets/card_faces/8_of_clubs.svg");
static EIGHT_OF_DIAMONDS: &[u8] = include_bytes!("../../assets/card_faces/8_of_diamonds.svg");
static EIGHT_OF_HEARTS: &[u8] = include_bytes!("../../assets/card_faces/8_of_hearts.svg");
static EIGHT_OF_SPADES: &[u8] = include_bytes!("../../assets/card_faces/8_of_spades.svg");

static NINE_OF_CLUBS: &[u8] = include_bytes!("../../assets/card_faces/9_of_clubs.svg");
static NINE_OF_DIAMONDS: &[u8] = include_bytes!("../../assets/card_faces/9_of_diamonds.svg");
static NINE_OF_HEARTS: &[u8] = include_bytes!("../../assets/card_faces/9_of_hearts.svg");
static NINE_OF_SPADES: &[u8] = include_bytes!("../../assets/card_faces/9_of_spades.svg");

static TEN_OF_CLUBS: &[u8] = include_bytes!("../../assets/card_faces/10_of_clubs.svg");
static TEN_OF_DIAMONDS: &[u8] = include_bytes!("../../assets/card_faces/10_of_diamonds.svg");
static TEN_OF_HEARTS: &[u8] = include_bytes!("../../assets/card_faces/10_of_hearts.svg");
static TEN_OF_SPADES: &[u8] = include_bytes!("../../assets/card_faces/10_of_spades.svg");

static ACE_OF_CLUBS: &[u8] = include_bytes!("../../assets/card_faces/ace_of_clubs.svg");
static ACE_OF_DIAMONDS: &[u8] = include_bytes!("../../assets/card_faces/ace_of_diamonds.svg");
static ACE_OF_HEARTS: &[u8] = include_bytes!("../../assets/card_faces/ace_of_hearts.svg");
static ACE_OF_SPADES: &[u8] = include_bytes!("../../assets/card_faces/ace_of_spades.svg");

static JACK_OF_CLUBS: &[u8] = include_bytes!("../../assets/card_faces/jack_of_clubs.svg");
static JACK_OF_DIAMONDS: &[u8] = include_bytes!("../../assets/card_faces/jack_of_diamonds.svg");
static JACK_OF_HEARTS: &[u8] = include_bytes!("../../assets/card_faces/jack_of_hearts.svg");
static JACK_OF_SPADES: &[u8] = include_bytes!("../../assets/card_faces/jack_of_spades.svg");

static KING_OF_CLUBS: &[u8] = include_bytes!("../../assets/card_faces/king_of_clubs.svg");
static KING_OF_DIAMONDS: &[u8] = include_bytes!("../../assets/card_faces/king_of_diamonds.svg");
static KING_OF_HEARTS: &[u8] = include_bytes!("../../assets/card_faces/king_of_hearts.svg");
static KING_OF_SPADES: &[u8] = include_bytes!("../../assets/card_faces/king_of_spades.svg");

static QUEEN_OF_CLUBS: &[u8] = include_bytes!("../../assets/card_faces/queen_of_clubs.svg");
static QUEEN_OF_DIAMONDS: &[u8] = include_bytes!("../../assets/card_faces/queen_of_diamonds.svg");
static QUEEN_OF_HEARTS: &[u8] = include_bytes!("../../assets/card_faces/queen_of_hearts.svg");
static QUEEN_OF_SPADES: &[u8] = include_bytes!("../../assets/card_faces/queen_of_spades.svg");

pub fn get_asset_data(asset_name: &str) -> Option<&[u8]> {
    match asset_name {
        "2_of_clubs" => Some(TWO_OF_CLUBS),
        "2_of_diamonds" => Some(TWO_OF_DIAMONDS),
        "2_of_hearts" => Some(TWO_OF_HEARTS),
        "2_of_spades" => Some(TWO_OF_SPADES),

        "3_of_clubs" => Some(THREE_OF_CLUBS),
        "3_of_diamonds" => Some(THREE_OF_DIAMONDS),
        "3_of_hearts" => Some(THREE_OF_HEARTS),
        "3_of_spades" => Some(THREE_OF_SPADES),

        "4_of_clubs" => Some(FOUR_OF_CLUBS),
        "4_of_diamonds" => Some(FOUR_OF_DIAMONDS),
        "4_of_hearts" => Some(FOUR_OF_HEARTS),
        "4_of_spades" => Some(FOUR_OF_SPADES),

        "5_of_clubs" => Some(FIVE_OF_CLUBS),
        "5_of_diamonds" => Some(FIVE_OF_DIAMONDS),
        "5_of_hearts" => Some(FIVE_OF_HEARTS),
        "5_of_spades" => Some(FIVE_OF_SPADES),

        "6_of_clubs" => Some(SIX_OF_CLUBS),
        "6_of_diamonds" => Some(SIX_OF_DIAMONDS),
        "6_of_hearts" => Some(SIX_OF_HEARTS),
        "6_of_spades" => Some(SIX_OF_SPADES),

        "7_of_clubs" => Some(SEVEN_OF_CLUBS),
        "7_of_diamonds" => Some(SEVEN_OF_DIAMONDS),
        "7_of_hearts" => Some(SEVEN_OF_HEARTS),
        "7_of_spades" => Some(SEVEN_OF_SPADES),

        "8_of_clubs" => Some(EIGHT_OF_CLUBS),
        "8_of_diamonds" => Some(EIGHT_OF_DIAMONDS),
        "8_of_hearts" => Some(EIGHT_OF_HEARTS),
        "8_of_spades" => Some(EIGHT_OF_SPADES),

        "9_of_clubs" => Some(NINE_OF_CLUBS),
        "9_of_diamonds" => Some(NINE_OF_DIAMONDS),
        "9_of_hearts" => Some(NINE_OF_HEARTS),
        "9_of_spades" => Some(NINE_OF_SPADES),

        "10_of_clubs" => Some(TEN_OF_CLUBS),
        "10_of_diamonds" => Some(TEN_OF_DIAMONDS),
        "10_of_hearts" => Some(TEN_OF_HEARTS),
        "10_of_spades" => Some(TEN_OF_SPADES),

        "ace_of_clubs" => Some(ACE_OF_CLUBS),
        "ace_of_diamonds" => Some(ACE_OF_DIAMONDS),
        "ace_of_hearts" => Some(ACE_OF_HEARTS),
        "ace_of_spades" => Some(ACE_OF_SPADES),

        "jack_of_clubs" => Some(JACK_OF_CLUBS),
        "jack_of_diamonds" => Some(JACK_OF_DIAMONDS),
        "jack_of_hearts" => Some(JACK_OF_HEARTS),
        "jack_of_spades" => Some(JACK_OF_SPADES),

        "king_of_clubs" => Some(KING_OF_CLUBS),
        "king_of_diamonds" => Some(KING_OF_DIAMONDS),
        "king_of_hearts" => Some(KING_OF_HEARTS),
        "king_of_spades" => Some(KING_OF_SPADES),

        "queen_of_clubs" => Some(QUEEN_OF_CLUBS),
        "queen_of_diamonds" => Some(QUEEN_OF_DIAMONDS),
        "queen_of_hearts" => Some(QUEEN_OF_HEARTS),
        "queen_of_spades" => Some(QUEEN_OF_SPADES),

        _ => None,
    }
}
