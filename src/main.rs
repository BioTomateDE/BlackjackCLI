#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::perf)]
#![warn(clippy::complexity)]
#![warn(clippy::cargo)]
#![warn(clippy::style)]
#![warn(clippy::nursery)]

mod action;
mod card;
mod deck;
mod flavor;
mod hand;
mod input;
mod sleep;

use action::Action;
use colored_print::cprintln;
use deck::Deck;
use hand::Hand;
use input::{get_bet, press_enter};
use sleep::sleep_ms;
use std::cmp::Ordering;

/// Returns the money gained (can be negative if lost).
fn play(mut bet: u64) -> i64 {
    let mut deck = Deck::new();
    let mut dealer_cards = Hand::new(deck.pop_card(), deck.pop_card());
    let mut player_cards = Hand::new(deck.pop_card(), deck.pop_card());

    sleep_ms(230);
    cprintln!("Dealer Upcard: %b^{}", dealer_cards.upcard());
    player_cards.print_info("Your");
    println!();

    // Player Blackjack; 3/2 payout.
    if player_cards.sum() == 21 {
        let payout = bet * 3 / 2;
        cprintln!("You won %b^{payout}%_^$ by getting a blackjack!");
        return payout as i64;
    }

    // Dealer Blackjack.
    if dealer_cards.sum() == 21 {
        dealer_cards.print_info("Dealer");
        cprintln!("%r:You lost %b^{bet}%_^$ because the dealer has a blackjack!");
        return -(bet as i64);
    }

    // Player action input until stand, double, 21 or busted.
    while player_cards.sum() < 21 {
        // TODO: check bet and balance to not go into debt
        let double_allowed: bool = player_cards.count() == 2;
        let action = Action::get_input(double_allowed);

        match action {
            Action::Stand => {
                cprintln!("You stood on %b^{}%_^.", player_cards.sum());
                break;
            }
            Action::Hit => player_cards.push_card(deck.pop_card()),
            Action::Double => {
                player_cards.push_card(deck.pop_card());
                player_cards.print_info("Your");
                bet *= 2;
                break;
            }
        }
        sleep_ms(187);
        player_cards.print_info("Your");
    }

    // Player busted.
    let sum: u8 = player_cards.sum();
    if sum > 21 {
        cprintln!("%r:You busted with a sum of %R:%b^{sum}%__%r:and lost %R:%b^{bet}%_^$%r:!");
        return -(bet as i64);
    }

    sleep_ms(320);
    dealer_cards.print_info("Dealer");

    // Dealer draws cards until they have a sum of 17 or higher.
    while dealer_cards.sum() < 17 {
        sleep_ms(800);
        dealer_cards.push_card(deck.pop_card());
        dealer_cards.print_info("Dealer");
    }

    // Dealer busted.
    let sum: u8 = dealer_cards.sum();
    if sum > 21 {
        cprintln!("%G:You won %b^{bet}%_^$ because the dealer busted with a sum of %b^{sum}%_^!");
        return bet as i64;
    }
    sleep_ms(530);

    let player_sum: u8 = player_cards.sum();
    let dealer_sum: u8 = dealer_cards.sum();

    match player_sum.cmp(&dealer_sum) {
        Ordering::Less => {
            cprintln!(
                "%R:You lost %b^{}%_^$ by having a lower card sum than the dealer %d^({} < {})%_^.",
                bet,
                player_sum,
                dealer_sum,
            );
            -(bet as i64)
        }
        Ordering::Equal => {
            cprintln!(
                "%y:You pushed by having the same card sum as the dealer %d^({})%_^.",
                player_sum
            );
            0
        }
        Ordering::Greater => {
            cprintln!(
                "%G:You won %b^{}%_^$ by having a higher card sum than the dealer %d^({} > {})%_^!",
                bet,
                player_sum,
                dealer_sum,
            );
            bet as i64
        }
    }
}

const HEADER: &str = "=============== Blackjack ===============";

fn main() {
    let mut balance: i64 = 1000;

    loop {
        cprintln!("%M:{HEADER}");
        cprintln!("Your balance: %b^{balance}%_^$");
        let bet: u64 = get_bet(balance as u64);
        let money_gained: i64 = play(bet);
        balance += money_gained;
        if balance < 2 {
            break;
        }
        cprintln!("%b^%w:[Press ENTER to restart]");
        press_enter();
    }

    sleep_ms(723);
    flavor::print_lose_message();
}
