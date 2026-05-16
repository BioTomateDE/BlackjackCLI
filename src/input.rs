use colored_print::{ceprint, cprintln};
use rand::{RngExt, rng};
use std::io::Write;
use std::io::{self, BufRead as _};

pub fn get_string_input(prompt: &str) -> String {
    ceprint!("%C:{prompt} > ");
    io::stderr().flush().expect("failed to flush stderr");

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read line from stdin");

    buffer.trim().to_string()
}

pub fn get_bet(balance: i64) -> i64 {
    loop {
        match try_get_bet(balance) {
            Ok(bet) => return bet,
            Err(err) => cprintln!("%R:{err}"),
        }
    }
}

fn try_get_bet(balance: i64) -> Result<i64, &'static str> {
    debug_assert!(balance > 1);

    let input: String = get_string_input("Choose your bet").to_ascii_lowercase();
    let bet: i64 = match input.as_str() {
        "h" | "half" => balance / 2,
        "a" | "all" => balance,
        "idk" | "rand" | "random" => rng().random_range(1..=balance / 2) * 2,
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

pub fn press_enter() {
    io::stdin().lock().read_line(&mut String::new()).unwrap();
}
