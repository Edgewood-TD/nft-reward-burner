use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap, Vector};
use near_sdk::json_types::U128;
use near_sdk::{
    env, log, near_bindgen, require, AccountId, Balance, BorshStorageKey, PanicOnDefault, Promise,
    PromiseOrValue, ONE_NEAR,
};
use std::collections::HashMap;
mod config;
mod constants;
mod mint;
mod payouts;
mod raffle;
mod utils;
mod views;
use constants::*;
use payouts::*;
use raffle::*;
use utils::*;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    royalties: LazyOption<Royalties>,

    max_supply: u128,
    whitelist: UnorderedMap<AccountId, u32>,
    free_mint_list: UnorderedMap<AccountId, u32>,

    mint_price: Balance,
    wl_price: Balance,
    available_nft: Raffle,

    sales_active: bool,
    pre_sale_active: bool,

    description: String,
    file_extension: String,
}
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    Royalties,
    //Custom
    Whitelist,
    FreeMintList,
    AvailableNft,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_init(
        owner_id: AccountId,
        mint_price: Balance,
        wl_price: Option<Balance>,
        max_supply: u128,
        nft_name: String,
        nft_symbol: String,
        icon: String,
        base_uri: String,
        description: String,
        file_extension: String,
    ) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: nft_name,
                symbol: nft_symbol,
                icon: Some(icon),
                base_uri: Some(base_uri),
                reference: None,
                reference_hash: None,
            },
            mint_price,
            wl_price,
            max_supply,
            description,
            file_extension,
        )
    }

    #[init]
    pub fn new(
        owner_id: AccountId,
        metadata: NFTContractMetadata,
        mint_price: Balance,
        wl_price: Option<Balance>,
        max_supply: u128,
        description: String,
        file_extension: String,
    ) -> Self {
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
            max_supply: max_supply,
            sales_active: false,
            pre_sale_active: false,
            whitelist: UnorderedMap::new(StorageKey::Whitelist.try_to_vec().unwrap()),
            royalties: LazyOption::new(StorageKey::Royalties, Some(&royalties)),
            mint_price: mint_price,
            wl_price: wl_price.unwrap_or(mint_price),
            free_mint_list: UnorderedMap::new(StorageKey::FreeMintList.try_to_vec().unwrap()),
            available_nft: Raffle::new(
                StorageKey::AvailableNft.try_to_vec().unwrap(),
                max_supply.try_into().unwrap(),
            ),
            description,
            file_extension,
        };

        this
    }
    #[init(ignore_state)]
    pub fn migrate(owner_id: AccountId) -> Self {
        let prev: Contract = env::state_read().expect("ERR_NOT_INITIALIZED");
        assert_eq!(
            prev.tokens.owner_id,
            env::signer_account_id(),
            "Only owner can call this method"
        );
        let mut perpetual_royalties: HashMap<AccountId, u8> = HashMap::new();
        perpetual_royalties.insert(owner_id, 100);
        let royalties: Royalties = Royalties {
            accounts: perpetual_royalties,
            percent: 5,
        };

        let metadata = prev.metadata.get().unwrap();
        // let prev_base_uri = prev.metadata.get().unwrap().base_uri.unwrap();
        let this = Contract {
            tokens: prev.tokens,
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            max_supply: prev.max_supply,
            whitelist: prev.whitelist,
            royalties: LazyOption::new(StorageKey::Royalties, Some(&royalties)),
            mint_price: prev.mint_price,
            wl_price: prev.wl_price,
            sales_active: prev.sales_active,
            pre_sale_active: prev.pre_sale_active,
            free_mint_list: prev.free_mint_list,
            available_nft: Raffle::new(
                StorageKey::AvailableNft.try_to_vec().unwrap(),
                prev.max_supply.try_into().unwrap(),
            ),
            description: prev.description,
            file_extension: prev.file_extension,
        };

        this
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
