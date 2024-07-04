use near_sdk::{
    collections::UnorderedSet, env, log, near, serde_json, store::LookupMap, AccountId, NearToken,
    PanicOnDefault, Promise,
};

const PUZZLE_NUMBER: u8 = 1;
const PRIZE_AMOUNT: NearToken = NearToken::from_near(1);
#[near(serializers = [json, borsh])]
pub struct JsonPuzzle {
    solution_hash: String,
    status: PuzzleStatus,
    answer: Vec<Answer>,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
struct CoordinatePair {
    x: u8,
    y: u8,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
enum AnswerDirection {
    Across,
    Down,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Answer {
    num: u8,
    start: CoordinatePair,
    direction: AnswerDirection,
    length: u8,
    clue: String,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub enum PuzzleStatus {
    Solved { memo: String },
    Unsolved,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
struct Puzzle {
    status: PuzzleStatus,
    answers: Vec<Answer>,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Crossword {
    owner_id: AccountId,
    puzzles: LookupMap<String, Puzzle>,
    unsolved_puzzles: UnorderedSet<String>,
}

#[near]
impl Crossword {
    #[init(ignore_state)]
    pub fn migrate(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            puzzles: LookupMap::new(b"c"),
            unsolved_puzzles: UnorderedSet::new(b"u"),
        }
    }

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            puzzles: LookupMap::new(b"p"),
            unsolved_puzzles: UnorderedSet::new(b"u"),
        }
    }

    pub fn new_puzzle(&mut self, solution_hash: String, answers: Vec<Answer>) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only the owner may call this function"
        );

        let existing = self.puzzles.insert(
            solution_hash.clone(),
            Puzzle {
                status: PuzzleStatus::Unsolved,
                answers,
            },
        );

        assert!(existing.is_none(), "Puzzle with that key already exists");
        self.unsolved_puzzles.insert(&solution_hash);
    }

    pub fn submit_solution(&mut self, solution: String, memo: String) -> Promise {
        let hash_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(hash_input);

        let mut puzzle = self
            .puzzles
            .get(&hashed_input_hex)
            .expect("ERR_NO_CORRECT_ANSWER")
            .clone();

        puzzle.status = match puzzle.status {
            PuzzleStatus::Unsolved => PuzzleStatus::Solved { memo: memo.clone() },
            _ => env::panic_str("ERR_PUZZLE_SOLVED"),
        };

        self.puzzles.insert(hashed_input_hex.clone(), puzzle);
        self.unsolved_puzzles.remove(&hashed_input_hex);

        log!(
            "Puzzle with solution hash {} solved, with memo: {}",
            hashed_input_hex,
            memo
        );

        Promise::new(env::predecessor_account_id()).transfer(PRIZE_AMOUNT)
    }

    pub fn get_puzzle_status(&self, solution_hash: String) -> &PuzzleStatus {
        &self
            .puzzles
            .get(&solution_hash)
            .expect("Puzzle doesn't exist")
            .status
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use near_sdk::test_utils::{get_logs, VMContextBuilder};
//     use near_sdk::{testing_env, AccountId};

//     #[test]
//     fn debug_get_hash() {
//         testing_env!(VMContextBuilder::new().build());

//         let debug_solution = "near nomicon ref finance";
//         let debugs_hash_bytes = near_sdk::env::sha256(debug_solution.as_bytes());
//         let debug_hash_string = hex::encode(debugs_hash_bytes);
//         println!("Let's debug: {:?}", debug_hash_string)
//     }

//     fn get_context(predecessor: AccountId) -> VMContextBuilder {
//         let mut builder = VMContextBuilder::new();
//         builder.predecessor_account_id(predecessor);
//         builder
//     }

//     #[test]
//     fn check_guess_solution() {
//         let alice: AccountId = "alice.testnet".parse().unwrap();

//         let context = get_context(alice);
//         testing_env!(context.build());

//         let contract = Crossword::new(
//             "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f".to_string(),
//         );

//         let mut guess_result = contract.guess_solution("wrong answer here".to_string());
//         assert!(!guess_result, "Expected failure from wrong guess");
//         assert_eq!(get_logs(), ["Try again."], "Expect failure log");
//         guess_result = contract.guess_solution("near nomicon ref finance".to_string());
//         assert!(guess_result, "Expected the correct answer to return true");
//         assert_eq!(
//             get_logs(),
//             ["Try again.", "You guessed right!"],
//             "Expected a successful log after the previous failed log."
//         );
//     }
// }
