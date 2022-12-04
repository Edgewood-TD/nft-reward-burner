use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::store::Vector;
use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, BorshStorageKey, PanicOnDefault, Promise,
    PromiseOrValue,
};
use std::collections::HashMap;
mod config;
mod constants;
mod mint;
mod nft_receiver;
mod payouts;
mod utils;
mod views;
use constants::*;
use payouts::*;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    royalties: LazyOption<Royalties>,

    max_supply: u128,
    mint_price: Balance,
    sales_active: bool,
    pre_sale_active: bool,
    description: String,
    file_extension: String,
    account_storage_balance: UnorderedMap<AccountId, U128>,
    accepted_nft: Option<AccountId>,
    burned_tokens: UnorderedMap<AccountId, Vector<TokenId>>,
}
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    Royalties,
    AccountStorageBalance,
    BurnedTokens,
    BurnedTokensInner { account_id: AccountId },
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_init(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: NFT_NAME.to_string(),
                symbol: NFT_SYMBOL.to_string(),
                icon: Some(ICON.to_string()),
                base_uri: Some(BASE_URI.to_string()),
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        metadata.assert_valid();
        let mut perpetual_royalties: HashMap<AccountId, u8> = HashMap::new();
        perpetual_royalties.insert(owner_id.clone(), 100);
        let royalties: Royalties = Royalties {
            accounts: perpetual_royalties,
            percent: 5,
        };
        let this = Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id.clone(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            //custom
            max_supply: MAX_SUPPLY,
            sales_active: false,
            pre_sale_active: false,
            account_storage_balance: UnorderedMap::new(
                StorageKey::AccountStorageBalance.try_to_vec().unwrap(),
            ),
            royalties: LazyOption::new(StorageKey::Royalties, Some(&royalties)),
            mint_price: MINT_PRICE,
            accepted_nft: None,
            description: DESCRIPTION.to_string(),
            file_extension: FILE_EXTENSION.to_string(),
            burned_tokens: UnorderedMap::new(StorageKey::BurnedTokens.try_to_vec().unwrap()),
        };

        this
    }
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) -> U128 {
        let account = account_id.unwrap_or_else(|| env::predecessor_account_id());
        let mut storage_balance = self
            .account_storage_balance
            .get(&account)
            .unwrap_or_else(|| {
                self.account_storage_balance
                    .insert(&account, &U128(0))
                    .unwrap();
                U128(0)
            });
        let attached_deposit = env::attached_deposit();
        if attached_deposit > 0 {
            storage_balance.0 += attached_deposit;
            self.account_storage_balance
                .insert(&account, &storage_balance)
                .unwrap();
        }
        storage_balance
    }
}
impl Contract {
    pub fn process_storage(&mut self, account_id: AccountId, init_storage: Balance) {
        let current_storage = env::storage_usage() as u128;
        if init_storage > current_storage {
            //REFUND
            let refund = init_storage - current_storage;
            let new_balance =
                U128(self.account_storage_balance.get(&account_id).unwrap().0 + refund);
            self.account_storage_balance
                .insert(&account_id, &new_balance);
        } else {
            //CHARGE
            let charge = current_storage - init_storage;
            let mut account_balance = self.account_storage_balance.get(&account_id).unwrap().0;
            if account_balance > charge {
                account_balance -= charge;
                self.account_storage_balance
                    .insert(&account_id, &U128(account_balance));
            } else {
                env::panic_str("not enough storage balance");
            }
        }
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
