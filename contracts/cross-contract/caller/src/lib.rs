#![no_std]
use soroban_sdk::{
    contract, contractclient, contractimpl, contracttype, symbol_short, Address, Env, Symbol,
};

const CALLEE: Symbol = symbol_short!("CALLEE");

/// Import the callee's data structure
#[contracttype]
#[derive(Clone)]
pub struct CalleeData {
    pub value: i128,
    pub caller: Address,
}

/// Client interface for the callee contract
#[contractclient(name = "CalleeClient")]
pub trait CalleeContractTrait {
    fn get_counter(env: Env) -> i128;
    fn increment(env: Env) -> i128;
    fn add_value(env: Env, value: i128) -> i128;
    fn get_balance(env: Env, user: Address) -> i128;
    fn get_stats(env: Env) -> (i128, Address);
    fn get_data(env: Env, caller: Address) -> CalleeData;
    fn multiply(env: Env, a: i128, b: i128) -> i128;
}

#[contract]
pub struct CallerContract;

#[contractimpl]
impl CallerContract {
    /// Initialize with callee contract address
    pub fn initialize(env: Env, callee: Address) {
        if env.storage().instance().has(&CALLEE) {
            panic!("already initialized");
        }

        env.storage().instance().set(&CALLEE, &callee);
    }

    /// Simple cross-contract call: read value
    pub fn call_get_counter(env: Env) -> i128 {
        let callee: Address = env
            .storage()
            .instance()
            .get(&CALLEE)
            .unwrap_or_else(|| panic!("not initialized"));

        // Create client and invoke
        let client = CalleeClient::new(&env, &callee);
        client.get_counter()
    }

    /// Cross-contract call: modify state
    pub fn call_increment(env: Env) -> i128 {
        let callee: Address = env
            .storage()
            .instance()
            .get(&CALLEE)
            .unwrap_or_else(|| panic!("not initialized"));

        let client = CalleeClient::new(&env, &callee);
        client.increment()
    }

    /// Cross-contract call with parameters
    pub fn call_add_value(env: Env, value: i128) -> i128 {
        let callee: Address = env
            .storage()
            .instance()
            .get(&CALLEE)
            .unwrap_or_else(|| panic!("not initialized"));

        let client = CalleeClient::new(&env, &callee);
        client.add_value(&value)
    }

    /// Cross-contract call with address parameter
    pub fn call_get_balance(env: Env, user: Address) -> i128 {
        let callee: Address = env
            .storage()
            .instance()
            .get(&CALLEE)
            .unwrap_or_else(|| panic!("not initialized"));

        let client = CalleeClient::new(&env, &callee);
        client.get_balance(&user)
    }

    /// Cross-contract call returning tuple
    pub fn call_get_stats(env: Env) -> (i128, Address) {
        let callee: Address = env
            .storage()
            .instance()
            .get(&CALLEE)
            .unwrap_or_else(|| panic!("not initialized"));

        let client = CalleeClient::new(&env, &callee);
        client.get_stats()
    }

    /// Cross-contract call returning struct
    pub fn call_get_data(env: Env) -> CalleeData {
        let callee: Address = env
            .storage()
            .instance()
            .get(&CALLEE)
            .unwrap_or_else(|| panic!("not initialized"));

        let caller_address = env.current_contract_address();
        let client = CalleeClient::new(&env, &callee);
        client.get_data(&caller_address)
    }

    /// Multiple cross-contract calls in sequence
    pub fn call_sequence(env: Env, value: i128) -> (i128, i128, i128) {
        let callee: Address = env
            .storage()
            .instance()
            .get(&CALLEE)
            .unwrap_or_else(|| panic!("not initialized"));

        let client = CalleeClient::new(&env, &callee);

        let initial = client.get_counter();
        let after_increment = client.increment();
        let after_add = client.add_value(&value);

        (initial, after_increment, after_add)
    }

    /// Cross-contract call with computation
    pub fn call_multiply(env: Env, a: i128, b: i128) -> i128 {
        let callee: Address = env
            .storage()
            .instance()
            .get(&CALLEE)
            .unwrap_or_else(|| panic!("not initialized"));

        let client = CalleeClient::new(&env, &callee);
        client.multiply(&a, &b)
    }

    /// Demonstrate composability: call multiply then add result
    pub fn call_and_process(env: Env, a: i128, b: i128) -> i128 {
        let callee: Address = env
            .storage()
            .instance()
            .get(&CALLEE)
            .unwrap_or_else(|| panic!("not initialized"));

        let client = CalleeClient::new(&env, &callee);

        // Multiply via cross-contract call
        let product = client.multiply(&a, &b);

        // Add to counter via cross-contract call
        client.add_value(&product)
    }

    /// Get the callee address
    pub fn get_callee(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&CALLEE)
            .unwrap_or_else(|| panic!("not initialized"))
    }
}
