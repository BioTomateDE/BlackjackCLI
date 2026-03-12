use colored_print::cprintln;
use rand::seq::IndexedRandom;

// You're welcome to add more objects here ^^
const OBJECTS_TO_SELL: &[&str] = &[
    "car",
    "house",
    "truck",
    "computer",
    "phone",
    "jewelry",
    "furniture",
    "pokemon cards",
    "lawnmower",
    "air fryer",
];

fn get_random_object() -> &'static str {
    OBJECTS_TO_SELL
        .choose(&mut rand::rng())
        .expect("Objects somehow empty")
}

pub fn print_lose_message() {
    let object: &str = get_random_object();
    cprintln!("%r:%i^You gambled away all your money! Time to sell your {object}%d^...");
}
