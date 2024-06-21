use near_sdk::{log, near};

#[near(contract_state)]
pub struct Contract {
    counter: u32,
}

impl Default for Contract {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

#[near]
impl Contract {
    #[init]
    #[private]
    pub fn init() -> Self {
        Self { counter: 0 }
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        Self { counter: 0 }
    }

    pub fn increment(&mut self) -> u32 {
        self.counter = self.counter + 1;
        log!("Increased number to {}", self.counter);
        self.counter
    }

    pub fn decrement(&mut self) -> u32 {
        self.counter = self.counter - 1;
        log!("Decreased number to {}", self.counter);
        self.counter
    }

    pub fn reset(&mut self) -> u32 {
        self.counter = 0;
        log!("Reset counter to zero");
        self.counter
    }

    pub fn get_counter(&self) -> u32 {
        self.counter
    }
}
