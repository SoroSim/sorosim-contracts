#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol};

const ADMIN: Symbol = symbol_short!("ADMIN");

/// Price data with timestamp
#[contracttype]
#[derive(Clone)]
pub struct PriceData {
    pub asset: String,
    pub price: i128,
    pub decimals: u32,
    pub timestamp: u64,
}

/// Storage keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Price(String), // asset symbol -> price data
}

#[contract]
pub struct OracleContract;

#[contractimpl]
impl OracleContract {
    /// Initialize the oracle with an admin
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }
        
        env.storage().instance().set(&ADMIN, &admin);
    }

    /// Set the price for an asset (admin only)
    pub fn set_price(
        env: Env,
        admin: Address,
        asset: String,
        price: i128,
        decimals: u32,
    ) {
        admin.require_auth();
        
        // Check admin authorization
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .unwrap_or_else(|| panic!("not initialized"));
        
        if stored_admin != admin {
            panic!("not the admin");
        }
        
        if price < 0 {
            panic!("price cannot be negative");
        }
        
        // Create price data with current timestamp
        let price_data = PriceData {
            asset: asset.clone(),
            price,
            decimals,
            timestamp: env.ledger().timestamp(),
        };
        
        // Store price data
        env.storage()
            .persistent()
            .set(&DataKey::Price(asset), &price_data);
    }

    /// Get the current price for an asset
    pub fn get_price(env: Env, asset: String) -> i128 {
        let price_data: PriceData = env
            .storage()
            .persistent()
            .get(&DataKey::Price(asset))
            .unwrap_or_else(|| panic!("price not set"));
        
        price_data.price
    }

    /// Get full price data including timestamp and decimals
    pub fn get_price_data(env: Env, asset: String) -> PriceData {
        env.storage()
            .persistent()
            .get(&DataKey::Price(asset))
            .unwrap_or_else(|| panic!("price not set"))
    }

    /// Get the age of the price data in seconds
    pub fn get_price_age(env: Env, asset: String) -> u64 {
        let price_data: PriceData = env
            .storage()
            .persistent()
            .get(&DataKey::Price(asset))
            .unwrap_or_else(|| panic!("price not set"));
        
        let current_time = env.ledger().timestamp();
        current_time.saturating_sub(price_data.timestamp)
    }

    /// Check if price data exists for an asset
    pub fn has_price(env: Env, asset: String) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Price(asset))
    }

    /// Get the admin address
    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&ADMIN)
            .unwrap_or_else(|| panic!("not initialized"))
    }

    /// Transfer admin role to a new address
    pub fn transfer_admin(env: Env, current_admin: Address, new_admin: Address) {
        current_admin.require_auth();
        
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .unwrap_or_else(|| panic!("not initialized"));
        
        if stored_admin != current_admin {
            panic!("not the admin");
        }
        
        env.storage().instance().set(&ADMIN, &new_admin);
    }
}

