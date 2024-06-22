use near_sdk::{env, json_types::U64, near, store::LookupMap, AccountId};

#[near(serializers = [json])]
#[derive(PartialEq, Eq)]
pub enum Coin {
    Tails,
    Heads,
}

#[near(contract_state)]
pub struct Contract {
    points: LookupMap<AccountId, U64>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            points: LookupMap::new(b"p"),
        }
    }
}

#[near]
impl Contract {
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        Self {
            points: LookupMap::new(b"p"),
        }
    }

    pub fn points_of(&self, player: AccountId) -> U64 {
        self.points.get(&player).unwrap_or(&U64::from(0)).clone()
    }

    pub fn flip_coin(&mut self, player_guess: Coin) -> bool {
        let random = *env::random_seed().get(0).unwrap() % 2;

        let current_points = u64::from(
            self.points
                .get(&env::predecessor_account_id())
                .unwrap_or(&U64::from(0))
                .clone(),
        );

        let result = match random {
            1 if player_guess == Coin::Heads => true,
            0 if player_guess == Coin::Tails => true,
            _ => false,
        };

        if result {
            let current_points = current_points + 1;
            self.points
                .insert(env::predecessor_account_id(), U64::from(current_points));
        };

        result
    }
}
