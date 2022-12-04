use super::*;

#[near_bindgen]
impl Contract {
    pub fn add_to_whitelist(&mut self, account_id: AccountId, amount: u32) {
        self.assert_owner(env::predecessor_account_id());
        self.whitelist.insert(&account_id, &amount);
    }
    pub fn add_to_whitelist_many(&mut self, account_ids: Vec<AccountId>, amounts: Vec<u32>) {
        self.assert_owner(env::predecessor_account_id());
        for (account_id, amount) in account_ids.into_iter().zip(amounts.into_iter()) {
            self.whitelist.insert(&account_id, &amount);
        }
    }
    pub fn is_whitelisted(&self, account_id: AccountId) -> bool {
        self.whitelist.keys().any(|x| x == account_id)
    }
}
