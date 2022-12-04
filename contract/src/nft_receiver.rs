use crate::*;

#[near_bindgen]
#[allow(unused_variables)]
impl Contract {
    pub fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> PromiseOrValue<bool> {
        require!(self.accepted_nft.is_none(), "ERR_ACCEPTED_NFT_NOT_SET");
        require!(
            self.accepted_nft.as_ref().unwrap() == &env::predecessor_account_id(),
            "ERR_NOT_ACCEPTED_NFT"
        );
        let mut burned_token = self
            .burned_tokens
            .get(&previous_owner_id)
            .unwrap_or_else(|| {
                Vector::new(
                    StorageKey::BurnedTokensInner {
                        account_id: previous_owner_id.clone(),
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });

        burned_token.push(token_id);
        if burned_token.len() >= 3 {
            self.internal_nft_mint(previous_owner_id);
        }

        PromiseOrValue::Value(false)
    }
}
