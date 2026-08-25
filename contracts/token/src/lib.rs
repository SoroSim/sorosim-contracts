#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    /// Mint new tokens to a recipient address
    pub fn mint(env: Env, to: Address, amount: i128) {
        to.require_auth();
        
        let balance = Self::balance(env.clone(), to.clone());
        env.storage()
            .persistent()
            .set(&to, &(balance + amount));
    }

    /// Transfer tokens from one address to another
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        
        let from_balance = Self::balance(env.clone(), from.clone());
        let to_balance = Self::balance(env.clone(), to.clone());
        
        if from_balance < amount {
            panic!("insufficient balance");
        }
        
        env.storage()
            .persistent()
            .set(&from, &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&to, &(to_balance + amount));
    }

    /// Get the token balance of an address
    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&account)
            .unwrap_or(0)
    }
}

