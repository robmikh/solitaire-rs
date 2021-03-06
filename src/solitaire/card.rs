use std::fmt::Display;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Hash)]
pub enum Face {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Hash)]
pub enum Suit {
    Diamond = 0,
    Spade = 1,
    Heart = 2,
    Club = 3,
}

#[derive(Hash)]
pub struct Card {
    pub face: Face,
    pub suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        string.push_str(match self.face {
            Face::Ace => "A",
            Face::Two => "2",
            Face::Three => "3",
            Face::Four => "4",
            Face::Five => "5",
            Face::Six => "6",
            Face::Seven => "7",
            Face::Eight => "8",
            Face::Nine => "9",
            Face::Ten => "10",
            Face::Jack => "J",
            Face::Queen => "Q",
            Face::King => "K",
        });
        string.push_str(match self.suit {
            Suit::Diamond => "♦",
            Suit::Spade => "♠",
            Suit::Heart => "♥",
            Suit::Club => "♣",
        });
        write!(f, "{}", string)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.face == other.face && self.suit == other.suit
    }
}

impl Eq for Card {}

impl Suit {
    pub fn is_red(&self) -> bool {
        match self {
            Suit::Diamond => true,
            Suit::Spade => false,
            Suit::Heart => true,
            Suit::Club => false,
        }
    }
}

impl Card {
    pub fn new(face: Face, suit: Suit) -> Self {
        Self { face, suit }
    }
}
