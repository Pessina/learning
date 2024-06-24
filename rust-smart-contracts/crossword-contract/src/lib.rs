use near_sdk::{env, near, PanicOnDefault};

const PUZZLE_NUMBER: u8 = 1;

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    crossword_solution: String,
}

#[near]
impl Contract {
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        Self {
            crossword_solution: "".into(),
        }
    }

    pub fn get_puzzle_number(&self) -> u8 {
        PUZZLE_NUMBER
    }

    pub fn set_solution(&mut self, solution: String) {
        self.crossword_solution = solution
    }

    pub fn guess_solution(&self, solution: String) {
        if solution == self.crossword_solution {
            env::log_str("You guessed right")
        } else {
            env::log_str("Try again.")
        }
    }
}
