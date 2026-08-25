#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

const TOKEN_A: Symbol = symbol_short!("TOKEN_A");
const TOKEN_B: Symbol = symbol_short!("TOKEN_B");
const RESERVE_A: Symbol = symbol_short!("RSRV_A");
const RESERVE_B: Symbol = symbol_short!("RSRV_B");
const TOTAL_SHARES: Symbol = symbol_short!("SHARES");

/// Storage keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Share(Address), // LP token shares per address
}

#[contract]
pub struct AmmContract;

#[contractimpl]
impl AmmContract {
    /// Initialize the AMM pool with two token addresses
    pub fn initialize(env: Env, token_a: Address, token_b: Address) {
        if env.storage().instance().has(&TOKEN_A) {
            panic!("already initialized");
        }
        
        env.storage().instance().set(&TOKEN_A, &token_a);
        env.storage().instance().set(&TOKEN_B, &token_b);
        env.storage().instance().set(&RESERVE_A, &0i128);
        env.storage().instance().set(&RESERVE_B, &0i128);
        env.storage().instance().set(&TOTAL_SHARES, &0i128);
    }

    /// Get the reserve amounts for both tokens
    pub fn get_reserves(env: Env) -> (i128, i128) {
        let reserve_a: i128 = env
            .storage()
            .instance()
            .get(&RESERVE_A)
            .unwrap_or(0);
        let reserve_b: i128 = env
            .storage()
            .instance()
            .get(&RESERVE_B)
            .unwrap_or(0);
        
        (reserve_a, reserve_b)
    }

    /// Get token addresses
    pub fn get_tokens(env: Env) -> (Address, Address) {
        let token_a: Address = env
            .storage()
            .instance()
            .get(&TOKEN_A)
            .unwrap_or_else(|| panic!("not initialized"));
        let token_b: Address = env
            .storage()
            .instance()
            .get(&TOKEN_B)
            .unwrap_or_else(|| panic!("not initialized"));
        
        (token_a, token_b)
    }

    /// Get LP token shares for an address
    pub fn get_shares(env: Env, address: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Share(address))
            .unwrap_or(0)
    }

    /// Get total LP token shares
    pub fn get_total_shares(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&TOTAL_SHARES)
            .unwrap_or(0)
    }

    /// Calculate the constant product (k = x * y)
    pub fn get_k(env: Env) -> i128 {
        let (reserve_a, reserve_b) = Self::get_reserves(env);
        reserve_a.checked_mul(reserve_b).unwrap_or(0)
    }

    /// Get the price of token A in terms of token B
    pub fn get_price_a(env: Env) -> i128 {
        let (reserve_a, reserve_b) = Self::get_reserves(env);
        if reserve_a == 0 {
            panic!("no liquidity");
        }
        reserve_b / reserve_a
    }

    /// Get the price of token B in terms of token A
    pub fn get_price_b(env: Env) -> i128 {
        let (reserve_a, reserve_b) = Self::get_reserves(env);
        if reserve_b == 0 {
            panic!("no liquidity");
        }
        reserve_a / reserve_b
    }
}

