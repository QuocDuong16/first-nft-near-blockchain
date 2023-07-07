pub mod metadata; // module

use metadata::{NFTContractMetadata, Token, TokenMetadata};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

pub type TokenId = String;

// Define the contract structure
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub token_by_id: LookupMap<TokenId, Token>,
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,
    pub metadata: Option<NFTContractMetadata>,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let metadata = NFTContractMetadata {
            spec: "nft-1.0.0".to_string(),
            name: "Blockchain Bootcamp Contract".to_string(),
            symbol: "BBC".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        };
        Self {
            owner_id: env::signer_account_id(),
            tokens_per_owner: LookupMap::new(b"tokens_per_owner".to_vec()),
            token_by_id: LookupMap::new(b"token_by_id".to_vec()),
            token_metadata_by_id: UnorderedMap::new(b"token_metadata_by_id".to_vec()),
            metadata: Some(metadata),
        }
    }

    #[payable]
    pub fn mint(
        &mut self,
        token_id: TokenId,
        title: Option<String>,
        description: Option<String>,
        media: Option<String>,
    ) {
        let token_metadata = TokenMetadata {
            title,
            description,
            media,
        };
        self.token_metadata_by_id.insert(&token_id, &token_metadata);
        let token = Token {
            token_id: token_id.clone(),
            owner_id: env::signer_account_id(),
            metadata: token_metadata,
        };
        self.token_by_id.insert(&token_id, &token);
        let mut token_set = self.tokens_per_owner.get(&env::signer_account_id()).unwrap_or(UnorderedSet::new(env::signer_account_id().as_bytes()));
        token_set.insert(&token_id);
        self.tokens_per_owner.insert(&env::signer_account_id(), &token_set);
    }

    pub fn get_token_by_id(&self, token_id: TokenId) -> Token {
        self.token_by_id.get(&token_id).unwrap()
    }

    pub fn get_tokens_per_owner(&self, owner_id: AccountId) -> Vec<Token> {
        let token_ids = self.tokens_per_owner.get(&owner_id).unwrap();
        let mut token_vec: Vec<Token> = Vec::new();
        for token_id in token_ids.iter() {
            token_vec.push(self.get_token_by_id(token_id));
        }
        token_vec
    }

    pub fn get_token_metadata_by_id(&self, token_id: TokenId) -> TokenMetadata {
        self.token_metadata_by_id.get(&token_id).unwrap()
    }
}
