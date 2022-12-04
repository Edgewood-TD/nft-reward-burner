use super::*;

#[near_bindgen]
impl Contract {
    pub fn update_uri(&mut self, uri: String) {
        self.assert_owner(env::signer_account_id());
        let prev: Contract = env::state_read().expect("ERR_NOT_INITIALIZED");
        let mut metadata = prev.metadata.get().unwrap();
        metadata.base_uri = Some(uri);

        self.metadata = LazyOption::new(StorageKey::Metadata.try_to_vec().unwrap(), Some(&metadata))
    }
    pub fn set_accepted_nft(&mut self, accepted_nft: AccountId) {
        self.assert_owner(env::signer_account_id());
        self.accepted_nft = Some(accepted_nft);
    }
    pub fn update_nft_name(&mut self, name: String) {
        self.assert_owner(env::signer_account_id());
        let mut metadata = self.metadata.get().unwrap();
        metadata.name = name;
        self.metadata = LazyOption::new(StorageKey::Metadata.try_to_vec().unwrap(), Some(&metadata))
    }
    pub fn flip_public_sale(&mut self) {
        self.assert_owner(env::signer_account_id());
        self.sales_active = !self.sales_active;
    }
    pub fn flip_presale(&mut self) {
        self.assert_owner(env::signer_account_id());
        self.pre_sale_active = !self.pre_sale_active;
    }
    pub fn transfer_ownership(&mut self, account_id: AccountId) {
        self.assert_owner(env::signer_account_id());
        self.tokens.owner_id = account_id;
    }
    pub fn set_mint_price(&mut self, price: Balance) {
        self.assert_owner(env::signer_account_id());
        self.mint_price = price;
    }
}
