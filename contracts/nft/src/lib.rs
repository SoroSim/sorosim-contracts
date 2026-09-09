#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol};

const TOKEN_COUNTER: Symbol = symbol_short!("COUNTER");

/// Storage key for token ownership: token_id -> owner
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Owner(u64),
    Metadata(u64),
}

#[contract]
pub struct NftContract;

#[contractimpl]
impl NftContract {
    /// Mint a new NFT to the specified address
    pub fn mint(env: Env, to: Address) -> u64 {
        to.require_auth();

        // Get next token ID
        let token_id: u64 = env.storage().instance().get(&TOKEN_COUNTER).unwrap_or(0);

        // Store owner
        env.storage()
            .persistent()
            .set(&DataKey::Owner(token_id), &to);

        // Increment counter
        env.storage()
            .instance()
            .set(&TOKEN_COUNTER, &(token_id + 1));

        token_id
    }

    /// Transfer an NFT to another address
    pub fn transfer(env: Env, token_id: u64, from: Address, to: Address) {
        from.require_auth();

        let owner = Self::owner_of(env.clone(), token_id);

        if owner != from {
            panic!("not the owner");
        }

        env.storage()
            .persistent()
            .set(&DataKey::Owner(token_id), &to);
    }

    /// Get the owner of a specific token
    pub fn owner_of(env: Env, token_id: u64) -> Address {
        env.storage()
            .persistent()
            .get(&DataKey::Owner(token_id))
            .unwrap_or_else(|| panic!("token does not exist"))
    }

    /// Get the total number of tokens minted
    pub fn total_supply(env: Env) -> u64 {
        env.storage().instance().get(&TOKEN_COUNTER).unwrap_or(0)
    }

    /// Mint a new NFT with metadata to the specified address
    pub fn mint_with_metadata(env: Env, to: Address, metadata: String) -> u64 {
        to.require_auth();

        // Get next token ID
        let token_id: u64 = env.storage().instance().get(&TOKEN_COUNTER).unwrap_or(0);

        // Store owner
        env.storage()
            .persistent()
            .set(&DataKey::Owner(token_id), &to);

        // Store metadata
        env.storage()
            .persistent()
            .set(&DataKey::Metadata(token_id), &metadata);

        // Increment counter
        env.storage()
            .instance()
            .set(&TOKEN_COUNTER, &(token_id + 1));

        token_id
    }

    /// Set or update metadata for an existing token
    pub fn set_metadata(env: Env, token_id: u64, from: Address, metadata: String) {
        from.require_auth();

        let owner = Self::owner_of(env.clone(), token_id);

        if owner != from {
            panic!("not the owner");
        }

        env.storage()
            .persistent()
            .set(&DataKey::Metadata(token_id), &metadata);
    }

    /// Get metadata for a specific token
    pub fn get_metadata(env: Env, token_id: u64) -> String {
        env.storage()
            .persistent()
            .get(&DataKey::Metadata(token_id))
            .unwrap_or_else(|| String::from_str(&env, ""))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_mint() {
        let env = Env::default();
        let contract_id = env.register_contract(None, NftContract);
        let client = NftContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        env.mock_all_auths();

        let token_id = client.mint(&user);
        assert_eq!(token_id, 0);
        assert_eq!(client.owner_of(&token_id), user);
        assert_eq!(client.total_supply(), 1);
    }

    #[test]
    fn test_transfer() {
        let env = Env::default();
        let contract_id = env.register_contract(None, NftContract);
        let client = NftContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let recipient = Address::generate(&env);
        env.mock_all_auths();

        let token_id = client.mint(&owner);
        client.transfer(&token_id, &owner, &recipient);

        assert_eq!(client.owner_of(&token_id), recipient);
    }

    #[test]
    fn test_mint_with_metadata() {
        let env = Env::default();
        let contract_id = env.register_contract(None, NftContract);
        let client = NftContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        env.mock_all_auths();

        let metadata = String::from_str(&env, "ipfs://QmTest123");
        let token_id = client.mint_with_metadata(&user, &metadata);

        assert_eq!(token_id, 0);
        assert_eq!(client.owner_of(&token_id), user);
        assert_eq!(client.get_metadata(&token_id), metadata);
    }

    #[test]
    fn test_set_metadata() {
        let env = Env::default();
        let contract_id = env.register_contract(None, NftContract);
        let client = NftContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        env.mock_all_auths();

        let token_id = client.mint(&owner);
        let metadata = String::from_str(&env, "ipfs://QmNewMetadata");

        client.set_metadata(&token_id, &owner, &metadata);
        assert_eq!(client.get_metadata(&token_id), metadata);
    }

    #[test]
    fn test_get_metadata_empty() {
        let env = Env::default();
        let contract_id = env.register_contract(None, NftContract);
        let client = NftContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        env.mock_all_auths();

        let token_id = client.mint(&user);
        let metadata = client.get_metadata(&token_id);

        assert_eq!(metadata, String::from_str(&env, ""));
    }

    #[test]
    #[should_panic(expected = "not the owner")]
    fn test_set_metadata_not_owner() {
        let env = Env::default();
        let contract_id = env.register_contract(None, NftContract);
        let client = NftContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let other = Address::generate(&env);
        env.mock_all_auths();

        let token_id = client.mint(&owner);
        let metadata = String::from_str(&env, "ipfs://QmTest");

        client.set_metadata(&token_id, &other, &metadata);
    }

    #[test]
    fn test_update_metadata() {
        let env = Env::default();
        let contract_id = env.register_contract(None, NftContract);
        let client = NftContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        env.mock_all_auths();

        let metadata1 = String::from_str(&env, "ipfs://QmFirst");
        let token_id = client.mint_with_metadata(&owner, &metadata1);
        assert_eq!(client.get_metadata(&token_id), metadata1);

        let metadata2 = String::from_str(&env, "ipfs://QmSecond");
        client.set_metadata(&token_id, &owner, &metadata2);
        assert_eq!(client.get_metadata(&token_id), metadata2);
    }

    #[test]
    fn test_metadata_persists_after_transfer() {
        let env = Env::default();
        let contract_id = env.register_contract(None, NftContract);
        let client = NftContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let recipient = Address::generate(&env);
        env.mock_all_auths();

        let metadata = String::from_str(&env, "ipfs://QmTest");
        let token_id = client.mint_with_metadata(&owner, &metadata);

        client.transfer(&token_id, &owner, &recipient);

        assert_eq!(client.owner_of(&token_id), recipient);
        assert_eq!(client.get_metadata(&token_id), metadata);
    }

    #[test]
    fn test_multiple_nfts_with_metadata() {
        let env = Env::default();
        let contract_id = env.register_contract(None, NftContract);
        let client = NftContractClient::new(&env, &contract_id);

        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        env.mock_all_auths();

        let metadata1 = String::from_str(&env, "ipfs://QmFirst");
        let metadata2 = String::from_str(&env, "ipfs://QmSecond");

        let token_id1 = client.mint_with_metadata(&user1, &metadata1);
        let token_id2 = client.mint_with_metadata(&user2, &metadata2);

        assert_eq!(client.get_metadata(&token_id1), metadata1);
        assert_eq!(client.get_metadata(&token_id2), metadata2);
        assert_eq!(client.total_supply(), 2);
    }
}
