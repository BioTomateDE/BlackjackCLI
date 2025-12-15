use std::fmt::{Display, Formatter};

use crate::card::{Card, CardNumber};

#[derive(Debug, Clone, PartialEq, Eq)]
/// A hand of cards (either player or dealer).
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn new(card1: Card, card2: Card) -> Self {
        Self(vec![card1, card2])
    }

    /// Get the first card of the hand ("upcard").
    /// This function will panic if the hand is empty (although this should never happen).
    pub fn upcard(&self) -> &Card {
        self.0.first().expect("Hand is empty")
    }

    /// How many cards this hand currently holds.
    pub fn count(&self) -> usize {
        self.0.len()
    }

    /// Add a card to this hand.
    pub fn push_card(&mut self, card: Card) {
        self.0.push(card);
    }

    /// Get the sum of the card values, accounting for "soft cards" (regarding aces).
    pub fn sum(&self) -> u8 {
        let mut sum: u8 = 0;
        for card in &self.0 {
            if card.number == CardNumber::Ace && sum >= 11 {
                sum += 1;
            } else {
                sum += card.number.value();
            }
        }
        sum
    }

    /// Prints out `Your Sum: {} | Your Cards: {}` for `who = "Your"`.
    pub fn print_info(&self, who: &str) {
        println!("{who} Sum: {} | {who} Cards: {}", self.sum(), self);
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (i, card) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", card)?;
        }

        write!(f, "]")
    }
}
