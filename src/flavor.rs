use colored_print::cprintln;
use rand::{rng, seq::IndexedRandom as _};

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

pub fn print_lose_message() {
    let obj: &str = OBJECTS_TO_SELL.choose(&mut rng()).unwrap();
    cprintln!("%r:%i^You gambled away all your money! Time to sell your {obj}%d^...");
}
