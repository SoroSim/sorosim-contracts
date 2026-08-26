#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

const TOKEN_COUNTER: Symbol = symbol_short!("COUNTER");

/// Storage key for token ownership: token_id -> owner
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Owner(u64),
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
}
