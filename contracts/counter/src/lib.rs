#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER_KEY: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct CounterContract;

#[contractimpl]
impl CounterContract {
    /// Increment the counter by 1 and return the new value
    pub fn increment(env: Env) -> i32 {
        let mut count: i32 = env.storage().temporary().get(&COUNTER_KEY).unwrap_or(0);
        count += 1;
        env.storage().temporary().set(&COUNTER_KEY, &count);
        count
    }

    /// Decrement the counter by 1 and return the new value
    pub fn decrement(env: Env) -> i32 {
        let mut count: i32 = env.storage().temporary().get(&COUNTER_KEY).unwrap_or(0);
        count -= 1;
        env.storage().temporary().set(&COUNTER_KEY, &count);
        count
    }

    /// Get the current counter value
    pub fn get(env: Env) -> i32 {
        env.storage().temporary().get(&COUNTER_KEY).unwrap_or(0)
    }
}

