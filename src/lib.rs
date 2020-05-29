use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128; // eventually we may use this for expiration
use near_sdk::{env, near_bindgen};
use borsh::schema::Definition::Tuple;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DebugU128 {}

impl Default for DebugU128 {
    fn default() -> Self {
        Self {}
    }
}

#[near_bindgen]
impl DebugU128 {
    pub fn small_u(&self, val: u128) -> (u128, Vec<u8>) {
        env::log(format!("(u128) I received: {}", val).as_bytes());
        (val, env::keccak256(format!("{}", val).as_bytes()))
    }

    pub fn big_u(&self, val: U128) -> (U128, Vec<u8>) {
        env::log(format!("(U128) I received: {:?}", val).as_bytes());
        let regular_num: u128 = val.into();
        env::log(format!("(U128 » u128) I turned this into: {}", regular_num).as_bytes());
        // env::keccak256(&regular_num.into())
        (val, env::keccak256(format!("{}", regular_num).as_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{MockedBlockchain};
    use near_sdk::{AccountId, testing_env, VMContext};

    fn alice() -> AccountId { "alice_near".to_string() }

    fn get_context() -> VMContext {
        VMContext {
            current_account_id: alice(),
            signer_account_id: alice(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: alice(),
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            epoch_height: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 300,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: true,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn call_small_u() {
        let context = get_context();
        testing_env!(context);
        let contract = DebugU128::default();

        let num: u128 = 1906293427246306700;
        let tuple = contract.small_u(num.clone());
        let returned_keccak = tuple.1;

        // keccak in test
        let test_keccak = env::keccak256(format!("{}", num).as_bytes());

        assert_eq!(returned_keccak, test_keccak);
        assert_eq!(tuple.0, num);
        println!("Same number back: {:?}", tuple.0);
        println!("Test keccak: {:?}", test_keccak);
    }

    #[test]
    fn call_big_u() {
        let context = get_context();
        testing_env!(context);
        let contract = DebugU128::default();

        let num: U128 = 1906293427246306700u128.into();
        let tuple = contract.big_u(num.clone());
        let returned_keccak = tuple.1;

        // keccak in test
        let regular_num: u128 = num.into();
        let test_keccak = env::keccak256(format!("{}", regular_num).as_bytes());

        assert_eq!(returned_keccak, test_keccak);
        assert_eq!(tuple.0, num);
        println!("Same number back: {:?}", tuple.0);
        println!("{:?}", test_keccak);
    }
}