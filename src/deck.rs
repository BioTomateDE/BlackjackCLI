use rand::rng;
use rand::seq::SliceRandom as _;

use crate::card::Card;
use crate::card::CardColor::*;
use crate::card::CardNumber::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deck(Vec<Card>);
impl Deck {
    /// Generates a new 52 card deck and shuffles it.
    pub fn new() -> Self {
        let mut deck: Vec<Card> = DECK.to_vec();
        deck.shuffle(&mut rng());
        Self(deck)
    }

    /// Pops the last card off the deck and returns it.
    /// This function will panic if the deck is empty.
    pub fn pop_card(&mut self) -> Card {
        self.0.pop().expect("Deck is empty")
    }
}

pub const DECK: [Card; 52] = [
    // Diamonds
    Card::new(Two, Diamonds),
    Card::new(Three, Diamonds),
    Card::new(Four, Diamonds),
    Card::new(Five, Diamonds),
    Card::new(Six, Diamonds),
    Card::new(Seven, Diamonds),
    Card::new(Eight, Diamonds),
    Card::new(Nine, Diamonds),
    Card::new(Ten, Diamonds),
    Card::new(Jack, Diamonds),
    Card::new(Queen, Diamonds),
    Card::new(King, Diamonds),
    Card::new(Ace, Diamonds),
    // Hearts
    Card::new(Two, Hearts),
    Card::new(Three, Hearts),
    Card::new(Four, Hearts),
    Card::new(Five, Hearts),
    Card::new(Six, Hearts),
    Card::new(Seven, Hearts),
    Card::new(Eight, Hearts),
    Card::new(Nine, Hearts),
    Card::new(Ten, Hearts),
    Card::new(Jack, Hearts),
    Card::new(Queen, Hearts),
    Card::new(King, Hearts),
    Card::new(Ace, Hearts),
    // Spades
    Card::new(Two, Spades),
    Card::new(Three, Spades),
    Card::new(Four, Spades),
    Card::new(Five, Spades),
    Card::new(Six, Spades),
    Card::new(Seven, Spades),
    Card::new(Eight, Spades),
    Card::new(Nine, Spades),
    Card::new(Ten, Spades),
    Card::new(Jack, Spades),
    Card::new(Queen, Spades),
    Card::new(King, Spades),
    Card::new(Ace, Spades),
    // Clubs
    Card::new(Two, Clubs),
    Card::new(Three, Clubs),
    Card::new(Four, Clubs),
    Card::new(Five, Clubs),
    Card::new(Six, Clubs),
    Card::new(Seven, Clubs),
    Card::new(Eight, Clubs),
    Card::new(Nine, Clubs),
    Card::new(Ten, Clubs),
    Card::new(Jack, Clubs),
    Card::new(Queen, Clubs),
    Card::new(King, Clubs),
    Card::new(Ace, Clubs),
];
