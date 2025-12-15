use std::fmt::{Display, Formatter};

use colored_print::cformat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardNumber {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl CardNumber {
    #[must_use]
    pub const fn value(self) -> u8 {
        match self {
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
            Self::Jack => 10,
            Self::Queen => 10,
            Self::King => 10,
            Self::Ace => 11,
        }
    }
}

impl Display for CardNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string: &str = match self {
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::Nine => "9",
            Self::Ten => "10",
            Self::Jack => "Jack",
            Self::Queen => "Queen",
            Self::King => "King",
            Self::Ace => "Ace",
        };
        write!(f, "{string}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardColor {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub number: CardNumber,
    pub color: CardColor,
}

impl Card {
    #[must_use]
    pub const fn new(number: CardNumber, color: CardColor) -> Self {
        Self { number, color }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self.color {
            CardColor::Diamonds => cformat!("%R:♦ "),
            CardColor::Hearts => cformat!("%R:♥ "),
            CardColor::Spades => cformat!("%W:♠ "),
            CardColor::Clubs => cformat!("%W:♣ "),
        };
        write!(f, "{string}")
    }
}
