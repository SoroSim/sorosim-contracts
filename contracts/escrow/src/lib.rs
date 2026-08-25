#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

const ESCROW_COUNTER: Symbol = symbol_short!("COUNTER");

/// Escrow status
#[contracttype]
#[derive(Clone, PartialEq)]
pub enum EscrowStatus {
    Active,
    Released,
    Refunded,
}

/// Escrow data structure
#[contracttype]
#[derive(Clone)]
pub struct Escrow {
    pub id: u64,
    pub depositor: Address,
    pub beneficiary: Address,
    pub amount: i128,
    pub release_time: u64,
    pub status: EscrowStatus,
}

/// Storage keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Escrow(u64),
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Create a new time-locked escrow
    pub fn deposit(
        env: Env,
        depositor: Address,
        beneficiary: Address,
        amount: i128,
        release_time: u64,
    ) -> u64 {
        depositor.require_auth();
        
        if amount <= 0 {
            panic!("amount must be positive");
        }
        
        if release_time <= env.ledger().timestamp() {
            panic!("release time must be in the future");
        }
        
        // Get next escrow ID
        let escrow_id: u64 = env
            .storage()
            .instance()
            .get(&ESCROW_COUNTER)
            .unwrap_or(0);
        
        // Create escrow
        let escrow = Escrow {
            id: escrow_id,
            depositor: depositor.clone(),
            beneficiary,
            amount,
            release_time,
            status: EscrowStatus::Active,
        };
        
        // Store escrow
        env.storage()
            .persistent()
            .set(&DataKey::Escrow(escrow_id), &escrow);
        
        // Increment counter
        env.storage()
            .instance()
            .set(&ESCROW_COUNTER, &(escrow_id + 1));
        
        escrow_id
    }

    /// Release funds to beneficiary after release time
    pub fn release(env: Env, escrow_id: u64) {
        // Get escrow
        let mut escrow: Escrow = env
            .storage()
            .persistent()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap_or_else(|| panic!("escrow does not exist"));
        
        // Check status
        if escrow.status != EscrowStatus::Active {
            panic!("escrow not active");
        }
        
        // Check release time
        if env.ledger().timestamp() < escrow.release_time {
            panic!("release time not reached");
        }
        
        // Mark as released
        escrow.status = EscrowStatus::Released;
        
        // Store updated escrow
        env.storage()
            .persistent()
            .set(&DataKey::Escrow(escrow_id), &escrow);
    }

    /// Refund to depositor before release time
    pub fn refund(env: Env, escrow_id: u64, depositor: Address) {
        depositor.require_auth();
        
        // Get escrow
        let mut escrow: Escrow = env
            .storage()
            .persistent()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap_or_else(|| panic!("escrow does not exist"));
        
        // Check status
        if escrow.status != EscrowStatus::Active {
            panic!("escrow not active");
        }
        
        // Check authorization
        if escrow.depositor != depositor {
            panic!("not the depositor");
        }
        
        // Check time (can only refund before release time)
        if env.ledger().timestamp() >= escrow.release_time {
            panic!("release time reached, use release instead");
        }
        
        // Mark as refunded
        escrow.status = EscrowStatus::Refunded;
        
        // Store updated escrow
        env.storage()
            .persistent()
            .set(&DataKey::Escrow(escrow_id), &escrow);
    }

    /// Get escrow details
    pub fn get_escrow(env: Env, escrow_id: u64) -> Escrow {
        env.storage()
            .persistent()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap_or_else(|| panic!("escrow does not exist"))
    }

    /// Check if escrow can be released
    pub fn can_release(env: Env, escrow_id: u64) -> bool {
        let escrow: Escrow = env
            .storage()
            .persistent()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap_or_else(|| panic!("escrow does not exist"));
        
        escrow.status == EscrowStatus::Active
            && env.ledger().timestamp() >= escrow.release_time
    }

    /// Get total number of escrows
    pub fn total_escrows(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&ESCROW_COUNTER)
            .unwrap_or(0)
    }
}

