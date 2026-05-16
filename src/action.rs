use crate::input::get_string_input;
use colored_print::cprintln;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Stand,
    Hit,
    Double,
}

impl Action {
    #[must_use]
    pub fn get_input(double_allowed: bool) -> Self {
        loop {
            let prompt: &str = if double_allowed {
                "Choose an action: [S]tand · [H]it · [D]ouble"
            } else {
                "Choose an action: [S]tand · [H]it"
            };

            let input: String = get_string_input(prompt).to_ascii_lowercase();
            let action: Self = match input.as_str() {
                "s" | "stand" => Self::Stand,
                "h" | "hit" => Self::Hit,
                "d" | "double" => Self::Double,
                "" => continue,
                _ => {
                    cprintln!("%R:Invalid action input!");
                    continue;
                }
            };

            if action == Self::Double && !double_allowed {
                cprintln!("%R:Doubling down is not allowed here!");
                continue;
            }

            break action;
        }
    }
}
