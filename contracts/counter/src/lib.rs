#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER_KEY: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct CounterContract;

#[contractimpl]
impl CounterContract {
    /// Increment the counter by 1 and return the new value
    pub fn increment(env: Env) -> i32 {
        let mut count: i32 = env.storage().instance().get(&COUNTER_KEY).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&COUNTER_KEY, &count);
        count
    }

    /// Decrement the counter by 1 and return the new value
    pub fn decrement(env: Env) -> i32 {
        let mut count: i32 = env.storage().instance().get(&COUNTER_KEY).unwrap_or(0);
        count -= 1;
        env.storage().instance().set(&COUNTER_KEY, &count);
        count
    }

    /// Get the current counter value
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
