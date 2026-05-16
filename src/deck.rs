use rand::{rng, seq::SliceRandom};

use crate::card::{Card, Rank, Suit};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deck(Vec<Card>);

impl Deck {
    /// Generates a new 52 card deck and shuffles it.
    pub fn new() -> Self {
        let mut deck: Vec<Card> = all_cards();
        deck.shuffle(&mut rng());
        Self(deck)
    }

    /// Pops the last card off the deck and returns it.
    /// This function will panic if the deck is empty.
    pub fn pop_card(&mut self) -> Card {
        self.0.pop().expect("Deck is empty")
    }
}

fn all_cards() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::with_capacity(52);

    for suit in Suit::all() {
        for rank in Rank::all() {
            deck.push(Card::new(rank, suit));
        }
    }

    deck
}
