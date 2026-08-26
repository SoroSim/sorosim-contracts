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
        env.storage().persistent().set(&to, &(balance + amount));
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
        env.storage().persistent().set(&to, &(to_balance + amount));
    }

    /// Get the token balance of an address
    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage().persistent().get(&account).unwrap_or(0)
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
