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

    /// An array of all ranks.
    #[must_use]
    pub const fn all() -> [Self; 13] {
        [
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
            Self::Ten,
            Self::Jack,
            Self::Queen,
            Self::King,
            Self::Ace,
        ]
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(self.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

impl Suit {
    /// An array of all suits.
    #[must_use]
    pub const fn all() -> [Self; 4] {
        [Self::Diamonds, Self::Hearts, Self::Spades, Self::Clubs]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    #[must_use]
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = self.rank;
        let string = match self.suit {
            Suit::Diamonds => cformat!("%R:♦ {r}"),
            Suit::Hearts => cformat!("%R:♥ {r}"),
            Suit::Spades => cformat!("%W:♠ {r}"),
            Suit::Clubs => cformat!("%W:♣ {r}"),
        };
        f.write_str(&string)
    }
}
