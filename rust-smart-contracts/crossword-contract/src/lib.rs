use near_sdk::{
    collections::UnorderedSet, env, ext_contract, log, near, serde_json, store::LookupMap,
    AccountId, Allowance, Gas, NearToken, PanicOnDefault, Promise, PromiseResult, PublicKey,
};

const PRIZE_AMOUNT: NearToken = NearToken::from_near(1);
const GAS_FOR_ACCOUNT_CREATION: Gas = Gas::from_gas(150_000_000_000_000);
const GAS_FOR_ACCOUNT_CALLBACK: Gas = Gas::from_gas(110_000_000_000_000);

#[ext_contract(ext_linkdrop)]
pub trait ExtLinkDropCrossContract {
    fn create_account(&mut self, new_account_id: AccountId, new_pk: PublicKey) -> Promise;
}

pub trait AfterClaim {
    fn call_back_after_create_account(
        &mut self,
        crossword_pk: PublicKey,
        new_acc_id: AccountId,
        memo: String,
        signer_account_pk: PublicKey,
    ) -> bool;

    fn call_back_after_transfer(
        &mut self,
        crossword_pk: PublicKey,
        receiver_acc_id: AccountId,
        memo: String,
        signer_account_pk: PublicKey,
    ) -> bool;
}

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
    Solved { solver_pk: PublicKey },
    Unsolved,
    Claimed { memo: String },
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
    puzzles: LookupMap<PublicKey, Puzzle>,
    unsolved_puzzles: UnorderedSet<PublicKey>,
    creator_account: AccountId,
}

#[near]
impl Crossword {
    #[init(ignore_state)]
    pub fn migrate(owner_id: AccountId, creator_account: AccountId) -> Self {
        Self {
            owner_id,
            puzzles: LookupMap::new(b"a"),
            unsolved_puzzles: UnorderedSet::new(b"b"),
            creator_account,
        }
    }

    #[init]
    pub fn new(owner_id: AccountId, creator_account: AccountId) -> Self {
        Self {
            owner_id,
            puzzles: LookupMap::new(b"p"),
            unsolved_puzzles: UnorderedSet::new(b"u"),
            creator_account,
        }
    }

    pub fn new_puzzle(&mut self, public_key: PublicKey, answers: Vec<Answer>) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only the owner may call this function"
        );

        let existing = self.puzzles.insert(
            public_key.clone(),
            Puzzle {
                status: PuzzleStatus::Unsolved,
                answers,
            },
        );

        assert!(existing.is_none(), "Puzzle with that key already exists");
        self.unsolved_puzzles.insert(&public_key);
    }

    pub fn submit_solution(&mut self, memo: String, solver_pk: PublicKey) -> Promise {
        // Old hashing code for solution
        // let hash_input = env::sha256(solution.as_bytes());
        // let hashed_input_hex = hex::encode(hash_input);

        let answer_pk = env::signer_account_pk();

        let mut puzzle = self
            .puzzles
            .get(&answer_pk)
            .expect("ERR_NO_CORRECT_ANSWER")
            .clone();

        puzzle.status = match puzzle.status {
            PuzzleStatus::Unsolved => PuzzleStatus::Solved {
                solver_pk: solver_pk.clone(),
            },
            _ => env::panic_str("ERR_PUZZLE_SOLVED"),
        };

        self.puzzles.insert(answer_pk.clone(), puzzle);
        self.unsolved_puzzles.remove(&answer_pk);

        log!(
            "Puzzle with solution hash {:?} solved, with memo: {}",
            answer_pk,
            memo
        );

        Promise::new(env::current_account_id()).add_access_key_allowance(
            solver_pk.into(),
            Allowance::limited(NearToken::from_yoctonear(250000000000000000000000)).unwrap(),
            env::current_account_id(),
            "claim_reward,claim_reward_new_account".to_string(),
        );

        Promise::new(env::current_account_id()).delete_key(answer_pk)
    }

    pub fn claim_reward_new_account(
        &self,
        crossword_pk: PublicKey,
        new_acc_id: AccountId,
        new_pk: PublicKey,
        memo: String,
    ) -> Promise {
        let signer_pk = env::signer_account_pk();

        let puzzle = self
            .puzzles
            .get(&crossword_pk)
            .expect("Puzzle doesn't exist");

        match puzzle.status {
            PuzzleStatus::Solved {
                solver_pk: ref puzzle_pk,
            } => {
                assert_eq!(puzzle_pk.clone(), signer_pk, "You're not the person who can claim this, or else you need to use your function-call access key, friend");
            }
            _ => env::panic_str("Puzzle should have `Solved` status to be claimed."),
        }

        ext_linkdrop::ext(self.creator_account.clone())
            .with_attached_deposit(PRIZE_AMOUNT)
            .with_static_gas(GAS_FOR_ACCOUNT_CREATION)
            .create_account(new_acc_id.clone(), new_pk)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_ACCOUNT_CALLBACK)
                    .call_back_after_create_account(
                        crossword_pk,
                        new_acc_id,
                        memo,
                        env::signer_account_pk(),
                    ),
            )
    }

    pub fn claim_reward(
        &self,
        crossword_pk: PublicKey,
        receiver_acc_id: AccountId,
        memo: String,
    ) -> Promise {
        let signer_pk = env::signer_account_pk();

        let puzzle = self
            .puzzles
            .get(&crossword_pk)
            .expect("That puzzle doesn't exist");

        match puzzle.status {
            PuzzleStatus::Solved {
                solver_pk: ref puzzle_pk,
            } => {
                assert_eq!(puzzle_pk.clone(), signer_pk, "You're not the person who can claim this, or else you need to use your function-call access key, friend");
            }
            _ => env::panic_str("Puzzle should have `Solved` status to be claimed."),
        };

        Promise::new(receiver_acc_id.clone())
            .transfer(PRIZE_AMOUNT)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_ACCOUNT_CALLBACK)
                    .call_back_after_transfer(
                        crossword_pk,
                        receiver_acc_id,
                        memo,
                        env::signer_account_pk(),
                    ),
            )
    }

    pub fn get_puzzle_status(&self, answer_pk: PublicKey) -> &PuzzleStatus {
        &self
            .puzzles
            .get(&answer_pk)
            .expect("Puzzle doesn't exist")
            .status
    }

    pub fn finalize_puzzle(
        &mut self,
        crossword_pk: PublicKey,
        account_id: AccountId,
        memo: String,
        signer_pk: PublicKey,
    ) {
        let mut puzzle = self
            .puzzles
            .get(&crossword_pk)
            .expect("Puzzle doesn't exist")
            .clone();

        puzzle.status = PuzzleStatus::Claimed { memo: memo.clone() };

        self.puzzles.insert(crossword_pk.clone(), puzzle);

        log!(
            "Puzzle with pk: {:?} claimed, new account created: {}, memo: {}, reward claimed: {}",
            crossword_pk,
            account_id,
            memo,
            PRIZE_AMOUNT
        );

        Promise::new(env::current_account_id()).delete_key(signer_pk);
    }
}

#[near]
impl AfterClaim for Crossword {
    #[private]
    fn call_back_after_create_account(
        &mut self,
        crossword_pk: PublicKey,
        new_acc_id: AccountId,
        memo: String,
        signer_account_pk: PublicKey,
    ) -> bool {
        assert_eq!(env::promise_results_count(), 1, "Expected 1 promise result");

        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                self.finalize_puzzle(crossword_pk, new_acc_id, memo, signer_account_pk);
                true
            }
            PromiseResult::Failed => false,
        }
    }

    #[private]
    fn call_back_after_transfer(
        &mut self,
        crossword_pk: PublicKey,
        receiver_acc_id: AccountId,
        memo: String,
        signer_account_pk: PublicKey,
    ) -> bool {
        assert_eq!(env::promise_results_count(), 1, "Expected 1 promise result");

        match env::promise_result(0) {
            PromiseResult::Successful(creation_result) => {
                let creation_succeeded: bool = serde_json::from_slice(&creation_result)
                    .expect("Could not turn result from account creation into boolean");
                if creation_succeeded {
                    self.finalize_puzzle(crossword_pk, receiver_acc_id, memo, signer_account_pk);
                    true
                } else {
                    false
                }
            }
            PromiseResult::Failed => todo!(),
        }
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
