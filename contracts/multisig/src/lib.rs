#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, Vec};

const OWNERS: Symbol = symbol_short!("OWNERS");
const THRESHOLD: Symbol = symbol_short!("THRESH");
const TX_COUNTER: Symbol = symbol_short!("TX_CNT");

/// Transaction status
#[contracttype]
#[derive(Clone, PartialEq)]
pub enum TxStatus {
    Pending,
    Executed,
    Cancelled,
}

/// Transaction proposal
#[contracttype]
#[derive(Clone)]
pub struct Transaction {
    pub id: u64,
    pub proposer: Address,
    pub to: Address,
    pub amount: i128,
    pub approvals: Vec<Address>,
    pub status: TxStatus,
}

/// Storage keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Transaction(u64),
}

#[contract]
pub struct MultisigContract;

#[contractimpl]
impl MultisigContract {
    /// Initialize the multisig wallet with owners and threshold
    pub fn initialize(env: Env, owners: Vec<Address>, threshold: u32) {
        if env.storage().instance().has(&OWNERS) {
            panic!("already initialized");
        }
        
        if owners.is_empty() {
            panic!("owners cannot be empty");
        }
        
        if threshold == 0 || threshold > owners.len() {
            panic!("invalid threshold");
        }
        
        env.storage().instance().set(&OWNERS, &owners);
        env.storage().instance().set(&THRESHOLD, &threshold);
        env.storage().instance().set(&TX_COUNTER, &0u64);
    }

    /// Propose a new transaction
    pub fn propose(env: Env, proposer: Address, to: Address, amount: i128) -> u64 {
        proposer.require_auth();
        
        // Check if proposer is an owner
        if !Self::is_owner(env.clone(), proposer.clone()) {
            panic!("not an owner");
        }
        
        // Get next transaction ID
        let tx_id: u64 = env
            .storage()
            .instance()
            .get(&TX_COUNTER)
            .unwrap_or(0);
        
        // Create transaction with proposer as first approval
        let mut approvals = Vec::new(&env);
        approvals.push_back(proposer.clone());
        
        let transaction = Transaction {
            id: tx_id,
            proposer,
            to,
            amount,
            approvals,
            status: TxStatus::Pending,
        };
        
        // Store transaction
        env.storage()
            .persistent()
            .set(&DataKey::Transaction(tx_id), &transaction);
        
        // Increment counter
        env.storage()
            .instance()
            .set(&TX_COUNTER, &(tx_id + 1));
        
        tx_id
    }

    /// Get transaction details
    pub fn get_transaction(env: Env, tx_id: u64) -> Transaction {
        env.storage()
            .persistent()
            .get(&DataKey::Transaction(tx_id))
            .unwrap_or_else(|| panic!("transaction does not exist"))
    }

    /// Get the list of owners
    pub fn get_owners(env: Env) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&OWNERS)
            .unwrap_or_else(|| panic!("not initialized"))
    }

    /// Get the approval threshold
    pub fn get_threshold(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&THRESHOLD)
            .unwrap_or_else(|| panic!("not initialized"))
    }

    /// Check if an address is an owner
    pub fn is_owner(env: Env, address: Address) -> bool {
        let owners: Vec<Address> = env
            .storage()
            .instance()
            .get(&OWNERS)
            .unwrap_or_else(|| panic!("not initialized"));
        
        for owner in owners.iter() {
            if owner == address {
                return true;
            }
        }
        false
    }

    /// Get total number of transactions
    pub fn total_transactions(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&TX_COUNTER)
            .unwrap_or(0)
    }
}

