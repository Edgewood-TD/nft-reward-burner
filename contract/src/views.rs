use super::*;
#[near_bindgen]
impl Contract {
    pub fn assert_owner(&self, account_id: AccountId) {
        require!(
            self.tokens.owner_id == account_id,
            "Only owner can call this method"
        );
    }
    pub fn get_owner(&self) -> AccountId {
        return self.tokens.owner_id.clone();
    }
    pub fn get_sale_status(&self) -> bool {
        return self.sales_active;
    }
    pub fn get_presale_status(&self) -> bool {
        return self.pre_sale_active;
    }
    pub fn get_mint_price(&self) -> Balance {
        return self.mint_price;
    }
    pub fn get_wl_mint_price(&self) -> Balance {
        return self.wl_price;
    }
    pub fn get_wl_amount(&self, account_id: AccountId) -> u32 {
        self.whitelist.get(&account_id).unwrap_or(0)
    }
}
