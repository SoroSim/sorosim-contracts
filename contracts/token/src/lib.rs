//! # Token Contract
//!
//! A fungible token implementation with transfer and allowance functionality.
//!
//! ## Features
//! - Mint tokens to addresses
//! - Transfer tokens between addresses
//! - Approve spending allowances
//! - Transfer tokens on behalf of another address (transferFrom)
//! - Query balances and allowances
//!
//! ## Storage
//! - Uses persistent storage for balances (Address -> i128)
//! - Uses persistent storage for allowances (AllowanceKey -> i128)
//!
//! ## Authorization
//! - `mint`: Requires recipient authorization
//! - `transfer`: Requires sender authorization
//! - `approve`: Requires owner authorization
//! - `transfer_from`: Requires spender authorization
//!
//! ## Example Usage
//! ```ignore
//! // Mint tokens
//! client.mint(&user, &1000);
//!
//! // Transfer tokens
//! client.transfer(&alice, &bob, &100);
//!
//! // Approve allowance
//! client.approve(&owner, &spender, &500);
//!
//! // Transfer using allowance
//! client.transfer_from(&spender, &owner, &recipient, &100);
//! ```

#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

/// Storage key for allowances mapping (owner, spender) to amount.
#[contracttype]
#[derive(Clone)]
pub struct AllowanceKey {
    /// The address that owns the tokens
    pub owner: Address,
    /// The address that is allowed to spend the tokens
    pub spender: Address,
}

/// Fungible token contract implementing basic ERC-20-like functionality.
#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    /// Mints new tokens to a recipient address.
    ///
    /// The minted tokens are added to the recipient's existing balance.
    ///
    /// # Arguments
    /// * `to` - The address to receive the minted tokens
    /// * `amount` - The amount of tokens to mint
    ///
    /// # Authorization
    /// Requires authentication from the `to` address.
    ///
    /// # Example
    /// ```ignore
    /// client.mint(&user_address, &1000); // Mints 1000 tokens to user
    /// ```
    pub fn mint(env: Env, to: Address, amount: i128) {
        to.require_auth();

        let balance = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&to, &(balance + amount));
    }

    /// Transfers tokens from one address to another.
    ///
    /// # Arguments
    /// * `from` - The address sending tokens
    /// * `to` - The address receiving tokens
    /// * `amount` - The amount of tokens to transfer
    ///
    /// # Authorization
    /// Requires authentication from the `from` address.
    ///
    /// # Panics
    /// Panics with "insufficient balance" if sender's balance is less than the amount.
    ///
    /// # Example
    /// ```ignore
    /// client.transfer(&alice, &bob, &100); // Alice sends 100 tokens to Bob
    /// ```
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
        env.storage().persistent().set(&to, &(to_balance + amount));
    }

    /// Returns the token balance of an address.
    ///
    /// # Arguments
    /// * `account` - The address to query
    ///
    /// # Returns
    /// The token balance, or 0 if the address has no balance.
    ///
    /// # Example
    /// ```ignore
    /// let balance = client.balance(&user); // Returns user's token balance
    /// ```
    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage().persistent().get(&account).unwrap_or(0)
    }

    /// Approves a spender to withdraw tokens on behalf of the owner.
    ///
    /// Sets the allowance for `spender` to withdraw up to `amount` tokens
    /// from `owner`'s account.
    ///
    /// # Arguments
    /// * `owner` - The address owning the tokens
    /// * `spender` - The address allowed to spend the tokens
    /// * `amount` - The maximum amount the spender can withdraw
    ///
    /// # Authorization
    /// Requires authentication from the `owner` address.
    ///
    /// # Example
    /// ```ignore
    /// client.approve(&owner, &spender, &500); // Allow spender to use 500 tokens
    /// ```
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
        env.storage().persistent().set(&to, &(to_balance + amount));

        // Decrease allowance
        let key = AllowanceKey {
            owner: from.clone(),
            spender: spender.clone(),
        };
        env.storage().persistent().set(&key, &(allowance - amount));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_mint() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        env.mock_all_auths();

        // Mint tokens
        client.mint(&user, &1000);
        assert_eq!(client.balance(&user), 1000);
    }

    #[test]
    fn test_multiple_mints() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        env.mock_all_auths();

        // Multiple mints should accumulate
        client.mint(&user, &500);
        client.mint(&user, &300);
        client.mint(&user, &200);
        assert_eq!(client.balance(&user), 1000);
    }

    #[test]
    fn test_balance_zero_by_default() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);

        // Balance should be 0 by default
        assert_eq!(client.balance(&user), 0);
    }

    #[test]
    fn test_transfer() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let from = Address::generate(&env);
        let to = Address::generate(&env);
        env.mock_all_auths();

        // Mint and transfer
        client.mint(&from, &1000);
        client.transfer(&from, &to, &300);

        assert_eq!(client.balance(&from), 700);
        assert_eq!(client.balance(&to), 300);
    }

    #[test]
    fn test_transfer_full_balance() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let from = Address::generate(&env);
        let to = Address::generate(&env);
        env.mock_all_auths();

        // Transfer full balance
        client.mint(&from, &1000);
        client.transfer(&from, &to, &1000);

        assert_eq!(client.balance(&from), 0);
        assert_eq!(client.balance(&to), 1000);
    }

    #[test]
    #[should_panic(expected = "insufficient balance")]
    fn test_transfer_insufficient_balance() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let from = Address::generate(&env);
        let to = Address::generate(&env);
        env.mock_all_auths();

        // Try to transfer more than balance
        client.mint(&from, &100);
        client.transfer(&from, &to, &200);
    }

    #[test]
    fn test_approve() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let spender = Address::generate(&env);
        env.mock_all_auths();

        // Approve spender
        client.approve(&owner, &spender, &500);
        assert_eq!(client.allowance(&owner, &spender), 500);
    }

    #[test]
    fn test_allowance_zero_by_default() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let spender = Address::generate(&env);

        // Allowance should be 0 by default
        assert_eq!(client.allowance(&owner, &spender), 0);
    }

    #[test]
    fn test_transfer_from() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let spender = Address::generate(&env);
        let recipient = Address::generate(&env);
        env.mock_all_auths();

        // Setup: mint, approve, transfer_from
        client.mint(&owner, &1000);
        client.approve(&owner, &spender, &500);
        client.transfer_from(&spender, &owner, &recipient, &300);

        assert_eq!(client.balance(&owner), 700);
        assert_eq!(client.balance(&recipient), 300);
        assert_eq!(client.allowance(&owner, &spender), 200);
    }

    #[test]
    fn test_transfer_from_full_allowance() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let spender = Address::generate(&env);
        let recipient = Address::generate(&env);
        env.mock_all_auths();

        // Use full allowance
        client.mint(&owner, &1000);
        client.approve(&owner, &spender, &500);
        client.transfer_from(&spender, &owner, &recipient, &500);

        assert_eq!(client.balance(&owner), 500);
        assert_eq!(client.balance(&recipient), 500);
        assert_eq!(client.allowance(&owner, &spender), 0);
    }

    #[test]
    #[should_panic(expected = "insufficient allowance")]
    fn test_transfer_from_insufficient_allowance() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let spender = Address::generate(&env);
        let recipient = Address::generate(&env);
        env.mock_all_auths();

        // Try to transfer more than allowance
        client.mint(&owner, &1000);
        client.approve(&owner, &spender, &100);
        client.transfer_from(&spender, &owner, &recipient, &200);
    }

    #[test]
    #[should_panic(expected = "insufficient balance")]
    fn test_transfer_from_insufficient_balance() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let spender = Address::generate(&env);
        let recipient = Address::generate(&env);
        env.mock_all_auths();

        // Allowance exceeds balance
        client.mint(&owner, &100);
        client.approve(&owner, &spender, &500);
        client.transfer_from(&spender, &owner, &recipient, &200);
    }

    #[test]
    fn test_multiple_approvals() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let spender1 = Address::generate(&env);
        let spender2 = Address::generate(&env);
        env.mock_all_auths();

        // Approve multiple spenders
        client.approve(&owner, &spender1, &300);
        client.approve(&owner, &spender2, &500);

        assert_eq!(client.allowance(&owner, &spender1), 300);
        assert_eq!(client.allowance(&owner, &spender2), 500);
    }

    #[test]
    fn test_approve_overwrite() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let spender = Address::generate(&env);
        env.mock_all_auths();

        // Overwrite previous approval
        client.approve(&owner, &spender, &300);
        client.approve(&owner, &spender, &500);

        assert_eq!(client.allowance(&owner, &spender), 500);
    }

    #[test]
    fn test_complex_scenario() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let charlie = Address::generate(&env);
        env.mock_all_auths();

        // Complex multi-user scenario
        client.mint(&alice, &1000);
        client.mint(&bob, &500);

        client.transfer(&alice, &bob, &200);
        assert_eq!(client.balance(&alice), 800);
        assert_eq!(client.balance(&bob), 700);

        client.approve(&bob, &charlie, &300);
        client.transfer_from(&charlie, &bob, &alice, &150);

        assert_eq!(client.balance(&alice), 950);
        assert_eq!(client.balance(&bob), 550);
        assert_eq!(client.allowance(&bob, &charlie), 150);
    }
}
