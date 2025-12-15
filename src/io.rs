use colored_print::*;
use rand::{Rng, rng};
use std::io;
use std::io::Write;

pub fn get_string_input(prompt: &str) -> String {
    ceprint!("%C:{prompt} > ");
    io::stderr().flush().expect("failed to flush stderr");

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read line from stdin");

    buffer.trim().to_string()
}

pub fn get_bet(balance: u64) -> u64 {
    loop {
        match try_get_bet(balance) {
            Ok(bet) => return bet,
            Err(err) => cprintln!("%R:{err}"),
        }
    }
}

fn try_get_bet(balance: u64) -> Result<u64, &'static str> {
    let input: String = get_string_input("Choose your bet");
    let bet: u64 = match input.as_str() {
        "half" => balance / 2,
        "all" => balance,
        "idk" => rng().random_range(2..=balance),
        _ => input
            .parse()
            .map_err(|_| "Please provide a valid number!")?,
    };

    if bet > balance {
        return Err("You cannot bet more than your balance!");
    }
    if bet < 2 {
        return Err("You must at least bet 2$!");
    }
    Ok(bet)
}
