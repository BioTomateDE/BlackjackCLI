use std::fmt::{Display, Formatter};

use crate::card::{Card, Rank};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    Player,
    Dealer,
}

impl Role {
    #[must_use]
    const fn whose(self) -> &'static str {
        match self {
            Self::Player => "Your",
            Self::Dealer => "Dealer",
        }
    }
}

/// A hand of cards (either player or dealer).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
    role: Role,
}

impl Hand {
    #[must_use]
    pub fn new(card1: Card, card2: Card, role: Role) -> Self {
        Self {
            cards: vec![card1, card2],
            role,
        }
    }

    #[must_use]
    pub fn new_player(card1: Card, card2: Card) -> Self {
        Self::new(card1, card2, Role::Player)
    }

    #[must_use]
    pub fn new_dealer(card1: Card, card2: Card) -> Self {
        Self::new(card1, card2, Role::Dealer)
    }

    /// Gets the first card of the hand (the "upcard").
    ///
    /// This function will panic if the hand is empty (although this should never happen).
    pub fn upcard(&self) -> Card {
        *self.cards.first().expect("Hand is empty")
    }

    /// How many cards this hand currently holds.
    pub const fn count(&self) -> usize {
        self.cards.len()
    }

    /// Adds a card to this hand.
    pub fn draw_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Gets the sum of the card values, accounting for "soft cards" (regarding aces).
    pub fn sum(&self) -> u8 {
        let mut sum: u8 = 0;
        for card in &self.cards {
            if card.number == Rank::Ace && sum >= 11 {
                sum += 1;
            } else {
                sum += card.number.value();
            }
        }
        sum
    }

    pub fn print_info(&self) {
        let who: &str = self.role.whose();
        println!("{who} Sum: {} | {who} Cards: {}", self.sum(), self);
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;

        for (i, card) in self.cards.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{card}")?;
        }

        f.write_str("]")
    }
}
