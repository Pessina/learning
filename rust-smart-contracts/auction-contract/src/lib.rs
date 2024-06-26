use near_sdk::{env, json_types::U64, near, AccountId, NearToken, PanicOnDefault, Promise};

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Bid {
    pub bidder: AccountId,
    pub bid: NearToken,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    highest_bid: Bid,
    auction_end_time: U64,
}

#[near]
impl Contract {
    #[init]
    #[private]
    pub fn init(end_time: U64) -> Self {
        Self {
            highest_bid: Bid {
                bidder: env::current_account_id(),
                bid: NearToken::from_yoctonear(1),
            },
            auction_end_time: end_time,
        }
    }

    #[payable]
    pub fn bid(&mut self) -> Promise {
        assert!(
            env::block_timestamp() < self.auction_end_time.into(),
            "Auction ended"
        );

        let bid = env::attached_deposit();
        let bidder = env::predecessor_account_id();

        let Bid {
            bidder: last_bidder,
            bid: last_bid,
        } = self.highest_bid.clone();

        assert!(bid > last_bid, "You must place a higher bid");

        self.highest_bid = Bid { bidder, bid };

        Promise::new(last_bidder).transfer(last_bid)
    }

    pub fn get_highest_bid(&self) -> Bid {
        self.highest_bid.clone()
    }

    pub fn get_auction_end_time(&self) -> U64 {
        self.auction_end_time
    }
}
