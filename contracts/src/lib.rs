use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, Promise, AccountId, Balance};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MusicNftExch {
    /// record proof belong to which an account
    proofs_map: LookupMap<String, AccountId>,

}

impl Default for MusicNftExch {
    fn default() -> Self {
        Self {
            proofs_map: LookupMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl MusicNftExch {
    pub fn insert_claim(&mut self, claim: String) {
        let account_id = env::signer_account_id();
        self.proofs_map.insert(&claim, &account_id);
    }

    pub fn get_claim_owner_id(&self, claim: String) -> Option<AccountId> {
        return self.proofs_map.get(&claim);
    }
    pub fn revoke_claim(&mut self, claim: String) {
        let account_id = env::signer_account_id();
        // todo need to consider fail condition
        let claim_own_id = self.proofs_map.get(&claim).unwrap();
        if account_id == claim_own_id {
            self.proofs_map.remove(&claim);
        }
    }

    pub fn transfer_claim(&mut self, claim: String, to: AccountId) {
        let account_id = env::signer_account_id();
        let claim_own_id = self.proofs_map.get(&claim).unwrap();
        if account_id == claim_own_id {
            self.proofs_map.insert(&claim, &to);
        }
    }

    pub fn transfer_claim_with(&mut self, to: AccountId, amount: Balance) {
        Promise::new(to)
            .transfer(amount * 1000000000000000000000);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn insert_claim() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = MusicNftExch::default();
        contract.insert_claim("music01".to_string());
        assert_eq!(Some("bob_near".to_string()), contract.get_claim_owner_id("music01".to_string()));
    }


    #[test]
    fn revoke_claim() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = MusicNftExch::default();
        contract.insert_claim("music01".to_string());

        contract.revoke_claim("music01".to_string());

        assert_eq!(None, contract.get_claim_owner_id("music01".to_string()));
    }


    #[test]
    fn transfer_claim() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = MusicNftExch::default();
        contract.insert_claim("music01".to_string());
        contract.transfer_claim("music01".to_string(), String::from("abort.near"));

        assert_eq!(Some("abort.near".to_string()), contract.get_claim_owner_id("music01".to_string()))
    }
}