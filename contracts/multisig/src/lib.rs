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

    /// Approve a pending transaction
    pub fn approve(env: Env, tx_id: u64, approver: Address) {
        approver.require_auth();
        
        // Check if approver is an owner
        if !Self::is_owner(env.clone(), approver.clone()) {
            panic!("not an owner");
        }
        
        // Get transaction
        let mut transaction: Transaction = env
            .storage()
            .persistent()
            .get(&DataKey::Transaction(tx_id))
            .unwrap_or_else(|| panic!("transaction does not exist"));
        
        // Check status
        if transaction.status != TxStatus::Pending {
            panic!("transaction not pending");
        }
        
        // Check if already approved
        for approval in transaction.approvals.iter() {
            if approval == approver {
                panic!("already approved");
            }
        }
        
        // Add approval
        transaction.approvals.push_back(approver);
        
        // Store updated transaction
        env.storage()
            .persistent()
            .set(&DataKey::Transaction(tx_id), &transaction);
    }

    /// Execute a transaction if threshold is met
    pub fn execute(env: Env, tx_id: u64) {
        // Get transaction
        let mut transaction: Transaction = env
            .storage()
            .persistent()
            .get(&DataKey::Transaction(tx_id))
            .unwrap_or_else(|| panic!("transaction does not exist"));
        
        // Check status
        if transaction.status != TxStatus::Pending {
            panic!("transaction not pending");
        }
        
        // Get threshold
        let threshold: u32 = env
            .storage()
            .instance()
            .get(&THRESHOLD)
            .unwrap_or_else(|| panic!("not initialized"));
        
        // Check if threshold is met
        if transaction.approvals.len() < threshold {
            panic!("threshold not met");
        }
        
        // Mark as executed
        transaction.status = TxStatus::Executed;
        
        // Store updated transaction
        env.storage()
            .persistent()
            .set(&DataKey::Transaction(tx_id), &transaction);
    }

    /// Cancel a pending transaction (only proposer can cancel)
    pub fn cancel(env: Env, tx_id: u64, canceller: Address) {
        canceller.require_auth();
        
        // Get transaction
        let mut transaction: Transaction = env
            .storage()
            .persistent()
            .get(&DataKey::Transaction(tx_id))
            .unwrap_or_else(|| panic!("transaction does not exist"));
        
        // Check status
        if transaction.status != TxStatus::Pending {
            panic!("transaction not pending");
        }
        
        // Check authorization (only proposer can cancel)
        if transaction.proposer != canceller {
            panic!("not the proposer");
        }
        
        // Mark as cancelled
        transaction.status = TxStatus::Cancelled;
        
        // Store updated transaction
        env.storage()
            .persistent()
            .set(&DataKey::Transaction(tx_id), &transaction);
    }

    /// Get the number of approvals for a transaction
    pub fn get_approval_count(env: Env, tx_id: u64) -> u32 {
        let transaction: Transaction = env
            .storage()
            .persistent()
            .get(&DataKey::Transaction(tx_id))
            .unwrap_or_else(|| panic!("transaction does not exist"));
        
        transaction.approvals.len()
    }

    /// Check if threshold is met for a transaction
    pub fn is_threshold_met(env: Env, tx_id: u64) -> bool {
        let transaction: Transaction = env
            .storage()
            .persistent()
            .get(&DataKey::Transaction(tx_id))
            .unwrap_or_else(|| panic!("transaction does not exist"));
        
        let threshold: u32 = env
            .storage()
            .instance()
            .get(&THRESHOLD)
            .unwrap_or_else(|| panic!("not initialized"));
        
        transaction.approvals.len() >= threshold
    }
}

