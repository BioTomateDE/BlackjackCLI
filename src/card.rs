use colored_print::cformat;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum Rank {
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

impl Rank {
    /// How much this card rank is worth.
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
            Self::Ten | Self::Jack | Self::Queen | Self::King => 10,
            Self::Ace => 11,
        }
    }

    /// The "name" of this card rank.
    ///
    /// This will be a number for most suits,
    /// except for Jack, Queen, King and Ace.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
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
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub number: Rank,
    pub suit: Suit,
}

impl Card {
    #[must_use]
    pub const fn new(number: Rank, suit: Suit) -> Self {
        Self { number, suit }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.number;
        let string = match self.suit {
            Suit::Diamonds => cformat!("%R:♦ {n}"),
            Suit::Hearts => cformat!("%R:♥ {n}"),
            Suit::Spades => cformat!("%W:♠ {n}"),
            Suit::Clubs => cformat!("%W:♣ {n}"),
        };
        f.write_str(&string)
    }
}
