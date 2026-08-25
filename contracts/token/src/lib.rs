#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

/// Storage key for allowances: (owner, spender)
#[contracttype]
#[derive(Clone)]
pub struct AllowanceKey {
    pub owner: Address,
    pub spender: Address,
}

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

    /// Approve a spender to withdraw up to a specified amount from the owner's account
    pub fn approve(env: Env, owner: Address, spender: Address, amount: i128) {
        owner.require_auth();
        
        let key = AllowanceKey {
            owner: owner.clone(),
            spender: spender.clone(),
        };
        
        env.storage().persistent().set(&key, &amount);
    }

    /// Get the allowance that a spender can withdraw from an owner
    pub fn allowance(env: Env, owner: Address, spender: Address) -> i128 {
        let key = AllowanceKey {
            owner: owner.clone(),
            spender: spender.clone(),
        };
        
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    /// Transfer tokens on behalf of another address using allowance
    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        
        let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        if allowance < amount {
            panic!("insufficient allowance");
        }
        
        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            panic!("insufficient balance");
        }
        
        let to_balance = Self::balance(env.clone(), to.clone());
        
        // Update balances
        env.storage()
            .persistent()
            .set(&from, &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&to, &(to_balance + amount));
        
        // Decrease allowance
        let key = AllowanceKey {
            owner: from.clone(),
            spender: spender.clone(),
        };
        env.storage()
            .persistent()
            .set(&key, &(allowance - amount));
    }
}

