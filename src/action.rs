use colored_print::cprintln;

use crate::io::get_string_input;

pub enum Action {
    Stand,
    Hit,
    Double,
}

impl Action {
    pub fn get_input(double_allowed: bool) -> Self {
        loop {
            let prompt: &str = if double_allowed {
                "Choose an action: [S]tand · [H]it · [D]ouble"
            } else {
                "Choose an action: [S]tand · [H]it"
            };

            let input: String = get_string_input(prompt);
            match input.to_ascii_lowercase().as_str() {
                "s" => return Self::Stand,
                "h" => return Self::Hit,
                "d" if double_allowed => return Self::Double,
                "d" => cprintln!("%R:Doubling down is not allowed here!"),
                _ => cprintln!("%R:Invalid action input!"),
            }
        }
    }
}
