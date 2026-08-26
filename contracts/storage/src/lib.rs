#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

// Instance storage keys (shared contract metadata)
const ADMIN: Symbol = symbol_short!("ADMIN");
const INIT_TIME: Symbol = symbol_short!("INITTIME");
const COUNTER: Symbol = symbol_short!("COUNTER");

// Persistent storage keys
#[contracttype]
#[derive(Clone)]
pub enum PersistentKey {
    Balance(Address),
    UserData(Address),
}

// Temporary storage keys
#[contracttype]
#[derive(Clone)]
pub enum TempKey {
    Session(Address),
    Cache(Symbol),
}

/// User data structure for persistent storage
#[contracttype]
#[derive(Clone)]
pub struct UserData {
    pub name: Symbol,
    pub score: i128,
    pub active: bool,
}

#[contract]
pub struct StorageContract;

#[contractimpl]
impl StorageContract {
    /// Initialize contract (uses instance storage)
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }

        // Instance storage: shared contract-level data
        env.storage().instance().set(&ADMIN, &admin);
        env.storage()
            .instance()
            .set(&INIT_TIME, &env.ledger().timestamp());
        env.storage().instance().set(&COUNTER, &0i128);
    }

    // ========== INSTANCE STORAGE ==========
    // Best for: Contract-level metadata, configuration
    // Lifetime: Lives as long as the contract instance
    // Cost: Medium (shared across contract)

    /// Increment global counter (instance storage)
    pub fn increment_counter(env: Env) -> i128 {
        let counter: i128 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        let new_value = counter + 1;
        env.storage().instance().set(&COUNTER, &new_value);
        new_value
    }

    /// Get global counter (instance storage)
    pub fn get_counter(env: Env) -> i128 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }

    /// Get admin address (instance storage)
    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&ADMIN)
            .unwrap_or_else(|| panic!("not initialized"))
    }

    /// Get initialization timestamp (instance storage)
    pub fn get_init_time(env: Env) -> u64 {
        env.storage().instance().get(&INIT_TIME).unwrap_or(0)
    }

    // ========== PERSISTENT STORAGE ==========
    // Best for: User balances, critical data that must persist
    // Lifetime: Permanent (until explicitly deleted or expired)
    // Cost: Higher (per-entry fees)

    /// Set user balance (persistent storage)
    pub fn set_balance(env: Env, user: Address, amount: i128) {
        user.require_auth();

        env.storage()
            .persistent()
            .set(&PersistentKey::Balance(user), &amount);
    }

    /// Get user balance (persistent storage)
    pub fn get_balance(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&PersistentKey::Balance(user))
            .unwrap_or(0)
    }

    /// Set user data (persistent storage with struct)
    pub fn set_user_data(env: Env, user: Address, name: Symbol, score: i128, active: bool) {
        user.require_auth();

        let data = UserData {
            name,
            score,
            active,
        };

        env.storage()
            .persistent()
            .set(&PersistentKey::UserData(user), &data);
    }

    /// Get user data (persistent storage)
    pub fn get_user_data(env: Env, user: Address) -> UserData {
        env.storage()
            .persistent()
            .get(&PersistentKey::UserData(user))
            .unwrap_or(UserData {
                name: symbol_short!("NONE"),
                score: 0,
                active: false,
            })
    }

    /// Remove user data (explicit deletion from persistent storage)
    pub fn remove_user_data(env: Env, user: Address) {
        user.require_auth();

        env.storage()
            .persistent()
            .remove(&PersistentKey::UserData(user));
    }

    /// Check if user data exists (persistent storage)
    pub fn has_user_data(env: Env, user: Address) -> bool {
        env.storage()
            .persistent()
            .has(&PersistentKey::UserData(user))
    }

    // ========== TEMPORARY STORAGE ==========
    // Best for: Session data, cache, temporary calculations
    // Lifetime: Short-lived (expires after ~5 minutes)
    // Cost: Lowest (cheap, auto-expires)

    /// Set session data (temporary storage)
    pub fn set_session(env: Env, user: Address, value: i128) {
        env.storage()
            .temporary()
            .set(&TempKey::Session(user), &value);
    }

    /// Get session data (temporary storage)
    pub fn get_session(env: Env, user: Address) -> i128 {
        env.storage()
            .temporary()
            .get(&TempKey::Session(user))
            .unwrap_or(0)
    }

    /// Set cache value (temporary storage)
    pub fn set_cache(env: Env, key: Symbol, value: i128) {
        env.storage().temporary().set(&TempKey::Cache(key), &value);
    }

    /// Get cache value (temporary storage)
    pub fn get_cache(env: Env, key: Symbol) -> i128 {
        env.storage()
            .temporary()
            .get(&TempKey::Cache(key))
            .unwrap_or(0)
    }

    /// Check if session exists (temporary storage)
    pub fn has_session(env: Env, user: Address) -> bool {
        env.storage().temporary().has(&TempKey::Session(user))
    }

    // ========== MIXED STORAGE OPERATIONS ==========
    // Demonstrates using multiple storage types together

    /// Complex operation using all three storage types
    pub fn process_transaction(env: Env, user: Address, amount: i128) {
        user.require_auth();

        // 1. Increment global counter (instance storage)
        let tx_count = Self::increment_counter(env.clone());

        // 2. Update user balance (persistent storage)
        let current_balance = Self::get_balance(env.clone(), user.clone());
        env.storage().persistent().set(
            &PersistentKey::Balance(user.clone()),
            &(current_balance + amount),
        );

        // 3. Cache transaction info (temporary storage)
        env.storage()
            .temporary()
            .set(&TempKey::Cache(symbol_short!("LAST_TX")), &tx_count);

        // 4. Update session (temporary storage)
        env.storage()
            .temporary()
            .set(&TempKey::Session(user), &amount);
    }

    /// Get storage summary
    pub fn get_storage_summary(env: Env, user: Address) -> (i128, i128, i128) {
        let counter = Self::get_counter(env.clone());
        let balance = Self::get_balance(env.clone(), user.clone());
        let session = Self::get_session(env, user);

        (counter, balance, session)
    }
}
