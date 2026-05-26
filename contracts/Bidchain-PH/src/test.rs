#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Env as _, Env, String};

    #[test]
    fn test_happy_path_submit_bid() {
        let env = Env::default();
        let contract_id = env.register(BidChainContract, ());

        let bid_id = String::from_str(&env, "BID001");
        let contractor = String::from_str(&env, "CONTRACTOR_A");
        let hash = String::from_str(&env, "HASH123");

        let result = BidChainContractClient::new(&env, &contract_id)
            .submit_bid(&bid_id, &contractor, &hash, &12345);

        assert!(result);
    }

    #[test]
    #[should_panic(expected = "Bid already exists")]
    fn test_duplicate_bid_failure() {
        let env = Env::default();
        let contract_id = env.register(BidChainContract, ());

        let bid_id = String::from_str(&env, "BID001");
        let contractor = String::from_str(&env, "CONTRACTOR_A");
        let hash = String::from_str(&env, "HASH123");

        let client = BidChainContractClient::new(&env, &contract_id);

        client.submit_bid(&bid_id, &contractor, &hash, &12345);
        client.submit_bid(&bid_id, &contractor, &hash, &12345); // duplicate
    }

    #[test]
    fn test_verify_bid_exists() {
        let env = Env::default();
        let contract_id = env.register(BidChainContract, ());

        let bid_id = String::from_str(&env, "BID002");
        let contractor = String::from_str(&env, "CONTRACTOR_B");
        let hash = String::from_str(&env, "HASH999");

        let client = BidChainContractClient::new(&env, &contract_id);

        client.submit_bid(&bid_id, &contractor, &hash, &12345);

        let exists = client.verify_bid(&bid_id);
        assert!(exists);
    }

    #[test]
    fn test_state_persistence_get_bid() {
        let env = Env::default();
        let contract_id = env.register(BidChainContract, ());

        let bid_id = String::from_str(&env, "BID003");
        let contractor = String::from_str(&env, "CONTRACTOR_C");
        let hash = String::from_str(&env, "HASH777");

        let client = BidChainContractClient::new(&env, &contract_id);

        client.submit_bid(&bid_id, &contractor, &hash, &99999);

        let bid = client.get_bid(&bid_id).unwrap();
        assert_eq!(bid.contractor_id, contractor);
    }

    #[test]
    fn test_missing_bid_returns_none() {
        let env = Env::default();
        let contract_id = env.register(BidChainContract, ());

        let bid_id = String::from_str(&env, "NON_EXISTENT");

        let client = BidChainContractClient::new(&env, &contract_id);

        let result = client.get_bid(&bid_id);
        assert!(result.is_none());
    }
}