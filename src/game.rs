use crate::{action::Action, deck::Deck, hand::Hand, sleep::sleep_ms};
use colored_print::cprintln;

#[derive(Debug, Clone)]
pub struct Game {
    balance: i64,
}

impl Game {
    /// Creates a new game with a default balance.
    #[must_use]
    pub const fn new() -> Self {
        Self { balance: 1000 }
    }

    #[must_use]
    pub const fn balance(&self) -> i64 {
        self.balance
    }

    /// Whether the player still has a positive balance.
    ///
    /// More accurately, this shows if another game can be played
    /// which requires a balance of at least 2.
    #[must_use]
    pub const fn has_balance(&self) -> bool {
        self.balance > 1
    }

    /// Plays one round of Blackjack in the terminal.
    ///
    /// The balance will be updated accordingly.
    pub fn play(&mut self, bet: i64) {
        debug_assert!(self.has_balance());
        debug_assert!(bet <= self.balance);

        let double_allowed: bool = self.balance >= 2 * bet;
        self.balance += play(bet, double_allowed);
    }
}

fn play(mut bet: i64, double_allowed: bool) -> i64 {
    let mut deck = Deck::new();
    let mut dealer_cards = Hand::new_dealer(deck.pop_card(), deck.pop_card());
    let mut player_cards = Hand::new_player(deck.pop_card(), deck.pop_card());

    sleep_ms(230);
    cprintln!("Dealer Upcard: %b^{}", dealer_cards.upcard());
    player_cards.print_info();
    println!();

    // Player Blackjack; 3/2 payout.
    if player_cards.sum() == 21 {
        let payout = bet * 3 / 2;
        cprintln!("You won %b^{payout}$%_^ by getting a blackjack!");
        return payout;
    }

    // Dealer Blackjack.
    if dealer_cards.sum() == 21 {
        dealer_cards.print_info();
        cprintln!("%r:You lost %b^{bet}$%_^ because the dealer has a blackjack!");
        return -bet;
    }

    // Player action input until stand, double, 21 or busted.
    while player_cards.sum() < 21 {
        let double_allowed: bool = double_allowed && player_cards.count() == 2;
        let action = Action::get_input(double_allowed);

        match action {
            Action::Stand => {
                cprintln!("You stood on %b^{}%_^.", player_cards.sum());
                break;
            }
            Action::Hit => player_cards.draw_card(deck.pop_card()),
            Action::Double => {
                player_cards.draw_card(deck.pop_card());
                player_cards.print_info();
                bet *= 2;
                break;
            }
        }
        sleep_ms(187);
        player_cards.print_info();
    }

    // Player busted.
    let sum: u8 = player_cards.sum();
    if sum > 21 {
        cprintln!("%r:You busted with a sum of %R:%b^{sum}%__ %r:and lost %R:%b^{bet}$%_^%r:!");
        return -bet;
    }

    sleep_ms(320);
    dealer_cards.print_info();

    // Dealer draws cards until they have a sum of 17 or higher.
    while dealer_cards.sum() < 17 {
        sleep_ms(800);
        dealer_cards.draw_card(deck.pop_card());
        dealer_cards.print_info();
    }

    // Dealer busted.
    let sum: u8 = dealer_cards.sum();
    if sum > 21 {
        cprintln!("%G:You won %b^{bet}$%_^ because the dealer busted with a sum of %b^{sum}%_^!");
        return bet;
    }
    sleep_ms(530);

    let player_sum: u8 = player_cards.sum();
    let dealer_sum: u8 = dealer_cards.sum();
    compare_sums(player_sum, dealer_sum, bet)
}

fn compare_sums(player_sum: u8, dealer_sum: u8, bet: i64) -> i64 {
    if player_sum < dealer_sum {
        cprintln!(
            "%R:You lost %b^{}$%_^ by having a lower card sum than the dealer %d^({} < {})%_^.",
            bet,
            player_sum,
            dealer_sum,
        );
        return -bet;
    }

    if player_sum > dealer_sum {
        cprintln!(
            "%G:You won %b^{}$%_^ by having a higher card sum than the dealer %d^({} > {})%_^!",
            bet,
            player_sum,
            dealer_sum,
        );
        return bet;
    }

    cprintln!(
        "%y:You pushed by having the same card sum as the dealer %d^({})%_^.",
        player_sum,
    );
    0
}
