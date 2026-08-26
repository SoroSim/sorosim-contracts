#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");
const ADMIN: Symbol = symbol_short!("ADMIN");

/// Data structure returned by callee
#[contracttype]
#[derive(Clone)]
pub struct CalleeData {
    pub value: i128,
    pub caller: Address,
}

/// Storage keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
}

#[contract]
pub struct CalleeContract;

#[contractimpl]
impl CalleeContract {
    /// Initialize the callee contract
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }
        
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&COUNTER, &0i128);
    }

    /// Simple getter function
    pub fn get_counter(env: Env) -> i128 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }

    /// Simple setter function
    pub fn increment(env: Env) -> i128 {
        let counter: i128 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        let new_value = counter + 1;
        env.storage().instance().set(&COUNTER, &new_value);
        new_value
    }

    /// Function with parameters
    pub fn add_value(env: Env, value: i128) -> i128 {
        let counter: i128 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        let new_value = counter + value;
        env.storage().instance().set(&COUNTER, &new_value);
        new_value
    }

    /// Function requiring authentication
    pub fn set_balance(env: Env, user: Address, amount: i128) {
        user.require_auth();
        
        env.storage()
            .persistent()
            .set(&DataKey::Balance(user), &amount);
    }

    /// Function returning balance
    pub fn get_balance(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(user))
            .unwrap_or(0)
    }

    /// Function returning multiple values (tuple)
    pub fn get_stats(env: Env) -> (i128, Address) {
        let counter: i128 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        let admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .unwrap_or_else(|| panic!("not initialized"));
        
        (counter, admin)
    }

    /// Function returning a struct
    pub fn get_data(env: Env, caller: Address) -> CalleeData {
        let counter: i128 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        
        CalleeData {
            value: counter,
            caller,
        }
    }

    /// Function that performs computation
    pub fn multiply(_env: Env, a: i128, b: i128) -> i128 {
        a.checked_mul(b).unwrap_or_else(|| panic!("overflow"))
    }

    /// Admin-only function
    pub fn reset_counter(env: Env, admin: Address) {
        admin.require_auth();
        
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .unwrap_or_else(|| panic!("not initialized"));
        
        if stored_admin != admin {
            panic!("not the admin");
        }
        
        env.storage().instance().set(&COUNTER, &0i128);
    }
}

