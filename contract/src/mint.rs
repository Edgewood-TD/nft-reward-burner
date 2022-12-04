use super::*;
use near_contract_standards::non_fungible_token::events;
use near_sdk::{require, ONE_NEAR};

impl Contract {
    pub fn internal_nft_mint(&mut self, receiver_id: AccountId) -> Token {
        let supply: U128 = self.tokens.nft_total_supply();

        require!(
            supply.0 < self.max_supply,
            "NFT total supply has reached maximum"
        );
        let token_id = (supply.0 + 1).to_string();

        /* let token_id = (supply.0 + 1).to_string(); */
        let token_metadata: TokenMetadata = TokenMetadata {
            copies: None,
            title: Some(format!("{}#{}", self.nft_metadata().name, token_id)),
            media: Some(MEDIA_LINK.to_string()),
            description: Some(self.description.clone()),
            expires_at: None,
            extra: None,
            issued_at: Some(env::block_timestamp().to_string()),
            reference: Some(METADATA_REFERENCE.to_string()),
            reference_hash: None,
            starts_at: None,
            media_hash: None,
            updated_at: None,
        };

        let token = self.tokens.internal_mint_with_refund(
            token_id,
            receiver_id,
            Some(token_metadata),
            Some(self.tokens.owner_id.clone()),
        );
        let event = events::NftMint {
            owner_id: &token.owner_id,
            memo: None,
            token_ids: &[&token.token_id],
        };
        event.emit();

        token
    }
}
