#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

use crate::game::Game;
use crate::input::{get_bet, press_enter};
use colored_print::cprintln;

mod action;
mod card;
mod deck;
mod flavor;
mod game;
mod hand;
mod input;
mod sleep;

const HEADER: &str = "=============== Blackjack ===============";

fn main() {
    let mut game = Game::new();

    while game.has_balance() {
        let bal: i64 = game.balance();
        cprintln!("%M:{HEADER}");
        cprintln!("Your balance: %b^{bal}$%_^");
        let bet = get_bet(bal);
        game.play(bet);
        cprintln!("%b^%w:[Press ENTER to restart]");
        press_enter();
    }

    flavor::print_lose_message();
}
