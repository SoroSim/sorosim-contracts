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
        let escrow_id: u64 = env.storage().instance().get(&ESCROW_COUNTER).unwrap_or(0);

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

        escrow.status == EscrowStatus::Active && env.ledger().timestamp() >= escrow.release_time
    }

    /// Get total number of escrows
    pub fn total_escrows(env: Env) -> u64 {
        env.storage().instance().get(&ESCROW_COUNTER).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_deposit() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);

        let contract_id = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &contract_id);

        let depositor = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        env.mock_all_auths();

        let release_time = 200;
        let escrow_id = client.deposit(&depositor, &beneficiary, &1000, &release_time);

        assert_eq!(escrow_id, 0);
        assert_eq!(client.total_escrows(), 1);

        let escrow = client.get_escrow(&escrow_id);
        assert_eq!(escrow.amount, 1000);
        assert_eq!(escrow.release_time, 200);
        assert_eq!(escrow.status, EscrowStatus::Active);
    }

    #[test]
    #[should_panic(expected = "amount must be positive")]
    fn test_deposit_zero_amount() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);

        let contract_id = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &contract_id);

        let depositor = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        env.mock_all_auths();

        client.deposit(&depositor, &beneficiary, &0, &200); // Should panic
    }

    #[test]
    #[should_panic(expected = "release time must be in the future")]
    fn test_deposit_past_release_time() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);

        let contract_id = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &contract_id);

        let depositor = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        env.mock_all_auths();

        client.deposit(&depositor, &beneficiary, &1000, &50); // Should panic
    }

    #[test]
    fn test_release() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);

        let contract_id = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &contract_id);

        let depositor = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        env.mock_all_auths();

        let release_time = 200;
        let escrow_id = client.deposit(&depositor, &beneficiary, &1000, &release_time);

        // Move time forward
        env.ledger().with_mut(|li| li.timestamp = 201);

        client.release(&escrow_id);

        let escrow = client.get_escrow(&escrow_id);
        assert_eq!(escrow.status, EscrowStatus::Released);
    }

    #[test]
    #[should_panic(expected = "release time not reached")]
    fn test_release_before_time() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);

        let contract_id = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &contract_id);

        let depositor = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        env.mock_all_auths();

        let release_time = 200;
        let escrow_id = client.deposit(&depositor, &beneficiary, &1000, &release_time);

        client.release(&escrow_id); // Should panic
    }

    #[test]
    fn test_refund() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);

        let contract_id = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &contract_id);

        let depositor = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        env.mock_all_auths();

        let release_time = 200;
        let escrow_id = client.deposit(&depositor, &beneficiary, &1000, &release_time);

        // Refund before release time
        client.refund(&escrow_id, &depositor);

        let escrow = client.get_escrow(&escrow_id);
        assert_eq!(escrow.status, EscrowStatus::Refunded);
    }

    #[test]
    #[should_panic(expected = "release time reached, use release instead")]
    fn test_refund_after_release_time() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);

        let contract_id = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &contract_id);

        let depositor = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        env.mock_all_auths();

        let release_time = 200;
        let escrow_id = client.deposit(&depositor, &beneficiary, &1000, &release_time);

        // Move past release time
        env.ledger().with_mut(|li| li.timestamp = 201);

        client.refund(&escrow_id, &depositor); // Should panic
    }

    #[test]
    #[should_panic(expected = "escrow not active")]
    fn test_release_after_refund() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);

        let contract_id = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &contract_id);

        let depositor = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        env.mock_all_auths();

        let release_time = 200;
        let escrow_id = client.deposit(&depositor, &beneficiary, &1000, &release_time);

        client.refund(&escrow_id, &depositor);

        env.ledger().with_mut(|li| li.timestamp = 201);
        client.release(&escrow_id); // Should panic
    }

    #[test]
    fn test_can_release() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);

        let contract_id = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &contract_id);

        let depositor = Address::generate(&env);
        let beneficiary = Address::generate(&env);
        env.mock_all_auths();

        let release_time = 200;
        let escrow_id = client.deposit(&depositor, &beneficiary, &1000, &release_time);

        assert_eq!(client.can_release(&escrow_id), false);

        env.ledger().with_mut(|li| li.timestamp = 200);
        assert_eq!(client.can_release(&escrow_id), true);

        client.release(&escrow_id);
        assert_eq!(client.can_release(&escrow_id), false);
    }

    #[test]
    fn test_multiple_escrows() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);

        let contract_id = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &contract_id);

        let depositor = Address::generate(&env);
        let beneficiary1 = Address::generate(&env);
        let beneficiary2 = Address::generate(&env);
        env.mock_all_auths();

        let escrow_id1 = client.deposit(&depositor, &beneficiary1, &1000, &200);
        let escrow_id2 = client.deposit(&depositor, &beneficiary2, &2000, &300);

        assert_eq!(escrow_id1, 0);
        assert_eq!(escrow_id2, 1);
        assert_eq!(client.total_escrows(), 2);

        let escrow1 = client.get_escrow(&escrow_id1);
        let escrow2 = client.get_escrow(&escrow_id2);

        assert_eq!(escrow1.amount, 1000);
        assert_eq!(escrow2.amount, 2000);
    }

    #[test]
    fn test_complex_scenario() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);

        let contract_id = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &contract_id);

        let depositor = Address::generate(&env);
        let beneficiary1 = Address::generate(&env);
        let beneficiary2 = Address::generate(&env);
        env.mock_all_auths();

        // Create two escrows
        let escrow_id1 = client.deposit(&depositor, &beneficiary1, &1000, &200);
        let escrow_id2 = client.deposit(&depositor, &beneficiary2, &2000, &300);

        // Refund first escrow
        client.refund(&escrow_id1, &depositor);
        assert_eq!(
            client.get_escrow(&escrow_id1).status,
            EscrowStatus::Refunded
        );

        // Release second escrow
        env.ledger().with_mut(|li| li.timestamp = 301);
        client.release(&escrow_id2);
        assert_eq!(
            client.get_escrow(&escrow_id2).status,
            EscrowStatus::Released
        );
    }
}
