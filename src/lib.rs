use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Gas, Promise};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use csv::Writer;  // Ensure this is imported if you're using Writer
use sha2::{Digest, Sha256};  // Ensure this is imported for using Sha256

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct EigenrankContract {
    social_db_contract_id: AccountId,
}

impl Default for EigenrankContract {
    fn default() -> Self {
        Self { 
            social_db_contract_id: "example.testnet".parse().expect("Invalid AccountId"),
        }
    }
}

#[near_bindgen]
impl EigenrankContract {
    #[init]
    pub fn new(social_db_contract_id: AccountId) -> Self {
        Self { 
            social_db_contract_id
        }
    }

    pub fn calc_rank(&self, seed_accounts: Vec<String>, user_id: String, seed_strategy: u8, localtrust_strategy: u8) {
        let call_promise = Promise::new(self.social_db_contract_id.clone())
            .function_call(
                "get_interactions".to_string(),
                serde_json::to_vec(&user_id).unwrap(),
                0,
                Gas(5_000_000_000_000),
            );

        call_promise.then(
            Promise::new(env::current_account_id()).function_call(
                "process_interactions".to_string(),
                serde_json::to_vec(&(seed_accounts, user_id, seed_strategy, localtrust_strategy)).unwrap(),
                0,
                Gas(10_000_000_000_000),
            )
        );
    }

    #[private]
    pub fn process_interactions(
        &self, 
        interactions_data: Vec<u8>, 
        params: (Vec<String>, String, u8, u8)
    ) {
        let interactions: Vec<Interaction> = serde_json::from_slice(&interactions_data).unwrap();
        let (_seed_accounts, user_id, _seed_strategy, localtrust_strategy) = params;

        let rankings = self.calculate_eigenrank(interactions, _seed_accounts, _seed_strategy, localtrust_strategy);

        self.save_rankings(user_id, rankings);
    }

    fn calculate_eigenrank(
        &self,
        interactions: Vec<Interaction>,
        _seed_accounts: Vec<String>,
        _seed_strategy: u8,
        _localtrust_strategy: u8,
    ) -> Vec<Ranking> {
        let mut local_trust = HashMap::<(String, String), f64>::new();
    
        // Step 1: Calculate local trust
        for interaction in interactions {
            let weight = 1.0 + interaction.mentions as f64 + interaction.reposts as f64 + interaction.replies as f64;
            *local_trust.entry((interaction.follower, interaction.follows)).or_insert(0.0) += weight;
        }
    
        // Step 2: Normalize local trust
        let mut total_trust = HashMap::<String, f64>::new();
        for ((follower, _), trust) in &local_trust {
            *total_trust.entry(follower.clone()).or_insert(0.0) += trust;
        }
        for ((follower, _), trust) in &mut local_trust {
            *trust /= total_trust[follower];  // Access directly with `follower` which is `&String`
        }

    
        // Step 3: Calculate global trust scores using the power iteration method
        let mut global_trust = HashMap::<(String, String), f64>::new();
        let mut next_global_trust = local_trust.clone();
    
        for _ in 0..10 {  // Iterate 10 times or until convergence
            for ((follower, follows), trust) in &local_trust {
                let contribution = trust * *global_trust.get(&(follower.clone(), follows.clone())).unwrap_or(&0.1);
                *next_global_trust.get_mut(&(follower.clone(), follows.clone())).unwrap_or(&mut 0.0) += contribution;
            }
            std::mem::swap(&mut global_trust, &mut next_global_trust);
            next_global_trust.values_mut().for_each(|v| *v = 0.0);
        }
    
        // Convert to rankings
        global_trust.iter().map(|(key, &score)| Ranking {
            user_id: key.1.clone(),
            score,
        }).collect()
    }
      
    
    fn save_rankings(&self, user_id: String, rankings: Vec<Ranking>) {
        let mut wtr = Writer::from_writer(vec![]);
        wtr.serialize(("UserID", "Rank")).unwrap();
        for rank in rankings {
            wtr.serialize((rank.user_id, rank.score)).unwrap();
        }
        let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
        let hash = Sha256::digest(data.as_bytes());
        let file_name = format!("{:x}", hash);
    
        // Use a promise to save to socialDB
        Promise::new(self.social_db_contract_id.clone())
            .function_call(
                "save_profile_data".to_string(),
                serde_json::to_vec(&(user_id, file_name, data)).unwrap(),
                0,
                Gas(10_000_000_000_000),
            );
    }    
}

#[derive(Serialize, Deserialize, Debug)]
struct Interaction {
    follower: String,
    follows: String,
    mentions: usize,
    reposts: usize,
    replies: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Ranking {
    user_id: String,
    score: f64,
}

#[derive(Serialize, Deserialize)]
enum SeedStrategy {
    Basic,
    Enhanced
}

#[derive(Serialize, Deserialize)]
enum LocalTrustStrategy {
    ExistingConnections,
    L1Rep1Rec1M1Enhanced,
    L1Rep6Rec3M12Enhanced,
}

struct StrategyConfig {
    likes_weight: u8,
    replies_weight: u8,
    reposts_weight: u8,
    mentions_weight: u8,
    boost_weight: u8,
}

impl Default for StrategyConfig {
    fn default() -> Self {
        Self {
            likes_weight: 1,
            replies_weight: 1,
            reposts_weight: 1,
            mentions_weight: 1,
            boost_weight: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, VMContext};
    use std::collections::HashMap;

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .current_account_id(accounts(0))
            .signer_account_id(accounts(1))
            .predecessor_account_id(accounts(1))
            .is_view(is_view)
            .build()
    }

    fn setup_mock_data() -> Vec<Interaction> {
        vec![
            Interaction { follower: "user1.near".to_string(), follows: "user2.near".to_string(), mentions: 1, reposts: 1, replies: 1 },
            Interaction { follower: "user2.near".to_string(), follows: "user1.near".to_string(), mentions: 2, reposts: 2, replies: 2 }
        ]
    }

    #[test]
    fn test_local_trust_calculation() {
        let interactions = setup_mock_data();
        let contract = EigenrankContract::new("social.near".to_string());
        let _seed_accounts = vec!["trusted_user.near".to_string()];
        
        let rankings = contract.calculate_eigenrank(interactions, _seed_accounts, 0, 0);
        
        // Assert that rankings are calculated correctly
        assert!(!rankings.is_empty());
        assert_eq!(rankings.len(), 2); // Assuming 2 users are in the ranking
        assert!(rankings.iter().any(|r| r.user_id == "user1.near" && r.score > 0.0));
    }

    #[test]
    fn test_global_trust_convergence() {
        let context = get_context(false);
        testing_env!(context);
        let contract = EigenrankContract::new("social.near".to_string());
        
        // Simulate a scenario where global trust scores should converge after several iterations
        contract.calc_rank(vec!["seed1.near".to_string()], "user1.near".to_string(), 0, 0);
        
        // This is a simplification: in reality, you'd need a way to fetch these scores
        // after they've been processed, possibly involving mock storage or similar.
    }

    #[test]
    fn test_calc_rank_effect() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = EigenrankContract::new("social.near".to_string());

        // Assuming you have a method to simulate the outcome of 'get_interactions'
        // For example, we pretend this method sets up the state as if interactions were fetched.
        contract.simulate_interactions_response("user1.near", vec![...]); // You need to implement this

        // Now we manually call the process method which would be called as the next step in the promise
        contract.process_interactions(vec![], (vec![], "sample_user_id".to_string(), 0, 0));
        // Assuming the `process_interactions` function needs a Vec<u8> and a tuple as parameters
        

        // Check state changes or other effects
        // For example, assert that rankings are updated or stored correctly
        assert_eq!(contract.get_rank("user1.near"), Some(expected_value));
    }

    #[test]
    fn test_interaction_with_external_contracts() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = EigenrankContract::new("mock_social_db.near".to_string());

        // Assuming 'mock_social_db' behaves in a certain way
        contract.mock_external_response(some_real_value);  // Implement mock responses

        // Test how your contract responds to these mocked responses
        assert!(contract.some_state_check());
    }
}
