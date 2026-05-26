#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Env, String, Symbol, Map, Vec, log,
};

const BID_MAP: Symbol = Symbol::short("BIDS");

#[contracttype]
#[derive(Clone)]
pub struct BidRecord {
    pub contractor_id: String,
    pub bid_hash: String,
    pub timestamp: u64,
}

#[contracttype]
pub enum DataKey {
    Bids,
}

#[contract]
pub struct BidChainContract;

#[contractimpl]
impl BidChainContract {

    /// Initializes storage (optional setup hook for clarity)
    pub fn init(env: Env) {
        env.storage().instance().set(&DataKey::Bids, &Map::<String, BidRecord>::new(&env));
    }

    /// MVP: Contractor submits bid hash
    /// Stores bid hash + contractor identity + timestamp on-chain
    pub fn submit_bid(
        env: Env,
        bid_id: String,
        contractor_id: String,
        bid_hash: String,
        timestamp: u64,
    ) -> bool {

        let mut bids: Map<String, BidRecord> =
            env.storage().instance()
                .get(&DataKey::Bids)
                .unwrap_or(Map::new(&env));

        // Prevent duplicate bid submission
        if bids.contains_key(bid_id.clone()) {
            panic!("Bid already exists");
        }

        let record = BidRecord {
            contractor_id,
            bid_hash,
            timestamp,
        };

        bids.set(bid_id, record);

        env.storage().instance().set(&DataKey::Bids, &bids);

        log!(&env, "Bid submitted successfully");

        true
    }

    /// Procurement officer verifies bid integrity and existence
    pub fn verify_bid(env: Env, bid_id: String) -> bool {
        let bids: Map<String, BidRecord> =
            env.storage().instance()
                .get(&DataKey::Bids)
                .unwrap_or(Map::new(&env));

        bids.contains_key(bid_id)
    }

    /// Fetch full bid record (for audit dashboard)
    pub fn get_bid(env: Env, bid_id: String) -> Option<BidRecord> {
        let bids: Map<String, BidRecord> =
            env.storage().instance()
                .get(&DataKey::Bids)
                .unwrap_or(Map::new(&env));

        bids.get(bid_id)
    }
}