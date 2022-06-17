use crate::*;

const INITIAL_BALANCE: Balance = 3_000_000_000_000_000_000_000_000; // 3e24yN, 3N

#[near_bindgen]
impl Contract {
    #[private]
    pub fn create_child_contract(prefix: AccountId, code: Vec<u8>) -> Promise {
        let subaccount_id =
            AccountId::try_from(format!("{}.{}", prefix, env::current_account_id()));
        Promise::new(subaccount_id.unwrap())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(INITIAL_BALANCE)
            .deploy_contract(code.to_vec())
    }
}
