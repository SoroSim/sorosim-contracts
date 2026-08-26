//! # Counter Contract
//!
//! A simple counter contract demonstrating basic state management in Soroban.
//!
//! ## Features
//! - Increment counter value
//! - Decrement counter value
//! - Query current counter value
//! - Persistent storage using instance storage
//!
//! ## Storage
//! Uses instance storage to maintain a single counter value that persists
//! across contract invocations.
//!
//! ## Example Usage
//! ```ignore
//! let client = CounterContractClient::new(&env, &contract_id);
//!
//! // Increment the counter
//! let value = client.increment(); // Returns 1
//!
//! // Get current value
//! let current = client.get(); // Returns 1
//!
//! // Decrement the counter
//! let value = client.decrement(); // Returns 0
//! ```

#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER_KEY: Symbol = symbol_short!("COUNTER");

/// Counter contract that maintains a single integer counter value.
///
/// The contract stores the counter in instance storage, which persists
/// for the lifetime of the contract instance.
#[contract]
pub struct CounterContract;

#[contractimpl]
impl CounterContract {
    /// Increments the counter by 1 and returns the new value.
    ///
    /// If the counter has never been set, it starts at 0 and returns 1.
    ///
    /// # Returns
    /// The new counter value after incrementing.
    ///
    /// # Example
    /// ```ignore
    /// let new_value = client.increment(); // If counter was 5, returns 6
    /// ```
    pub fn increment(env: Env) -> i32 {
        let mut count: i32 = env.storage().instance().get(&COUNTER_KEY).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&COUNTER_KEY, &count);
        count
    }

    /// Decrements the counter by 1 and returns the new value.
    ///
    /// If the counter has never been set, it starts at 0 and returns -1.
    /// The counter can go negative.
    ///
    /// # Returns
    /// The new counter value after decrementing.
    ///
    /// # Example
    /// ```ignore
    /// let new_value = client.decrement(); // If counter was 5, returns 4
    /// ```
    pub fn decrement(env: Env) -> i32 {
        let mut count: i32 = env.storage().instance().get(&COUNTER_KEY).unwrap_or(0);
        count -= 1;
        env.storage().instance().set(&COUNTER_KEY, &count);
        count
    }

    /// Returns the current counter value without modifying it.
    ///
    /// If the counter has never been set, returns 0.
    ///
    /// # Returns
    /// The current counter value.
    ///
    /// # Example
    /// ```ignore
    /// let value = client.get(); // Returns current counter value
    /// ```
    pub fn get(env: Env) -> i32 {
        env.storage().instance().get(&COUNTER_KEY).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_initial_value() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CounterContract);
        let client = CounterContractClient::new(&env, &contract_id);

        // Initial value should be 0
        assert_eq!(client.get(), 0);
    }

    #[test]
    fn test_increment() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CounterContract);
        let client = CounterContractClient::new(&env, &contract_id);

        // Increment once
        let result = client.increment();
        assert_eq!(result, 1);
        assert_eq!(client.get(), 1);
    }

    #[test]
    fn test_multiple_increments() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CounterContract);
        let client = CounterContractClient::new(&env, &contract_id);

        // Increment multiple times
        assert_eq!(client.increment(), 1);
        assert_eq!(client.increment(), 2);
        assert_eq!(client.increment(), 3);
        assert_eq!(client.get(), 3);
    }

    #[test]
    fn test_decrement() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CounterContract);
        let client = CounterContractClient::new(&env, &contract_id);

        // Start from 0 and decrement
        let result = client.decrement();
        assert_eq!(result, -1);
        assert_eq!(client.get(), -1);
    }

    #[test]
    fn test_increment_and_decrement() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CounterContract);
        let client = CounterContractClient::new(&env, &contract_id);

        // Increment then decrement
        client.increment();
        client.increment();
        assert_eq!(client.get(), 2);

        client.decrement();
        assert_eq!(client.get(), 1);

        client.decrement();
        assert_eq!(client.get(), 0);
    }

    #[test]
    fn test_decrement_below_zero() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CounterContract);
        let client = CounterContractClient::new(&env, &contract_id);

        // Decrement below zero
        client.decrement();
        client.decrement();
        client.decrement();
        assert_eq!(client.get(), -3);
    }

    #[test]
    fn test_large_sequence() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CounterContract);
        let client = CounterContractClient::new(&env, &contract_id);

        // Large sequence of operations
        for i in 1..=10 {
            assert_eq!(client.increment(), i);
        }

        for i in (5..10).rev() {
            assert_eq!(client.decrement(), i);
        }

        assert_eq!(client.get(), 5);
    }

    #[test]
    fn test_persistence() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CounterContract);
        let client = CounterContractClient::new(&env, &contract_id);

        // Set value
        client.increment();
        client.increment();
        client.increment();

        // Value should persist
        assert_eq!(client.get(), 3);
        assert_eq!(client.get(), 3); // Multiple reads

        // Continue incrementing
        assert_eq!(client.increment(), 4);
    }
}
