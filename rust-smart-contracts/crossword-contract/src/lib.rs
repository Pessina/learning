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

    #[init]
    pub fn new(solution: String) -> Self {
        Self {
            crossword_solution: solution,
        }
    }

    pub fn get_puzzle_number(&self) -> u8 {
        PUZZLE_NUMBER
    }

    pub fn set_solution(&mut self, solution: String) {
        self.crossword_solution = solution
    }

    pub fn get_solution(&self) -> String {
        self.crossword_solution.clone()
    }

    pub fn guess_solution(&self, solution: String) -> bool {
        let hash = hex::encode(env::sha256(solution.as_bytes()));

        if hash == self.crossword_solution {
            env::log_str("You guessed right!");
            true
        } else {
            env::log_str("Try again.");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    #[test]
    fn debug_get_hash() {
        testing_env!(VMContextBuilder::new().build());

        let debug_solution = "near nomicon ref finance";
        let debugs_hash_bytes = near_sdk::env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debugs_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string)
    }

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn check_guess_solution() {
        let alice: AccountId = "alice.testnet".parse().unwrap();

        let context = get_context(alice);
        testing_env!(context.build());

        let contract = Contract::new(
            "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f".to_string(),
        );

        let mut guess_result = contract.guess_solution("wrong answer here".to_string());
        assert!(!guess_result, "Expected failure from wrong guess");
        assert_eq!(get_logs(), ["Try again."], "Expect failure log");
        guess_result = contract.guess_solution("near nomicon ref finance".to_string());
        assert!(guess_result, "Expected the correct answer to return true");
        assert_eq!(
            get_logs(),
            ["Try again.", "You guessed right!"],
            "Expected a successful log after the previous failed log."
        );
    }
}
