#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

const ADMIN: Symbol = symbol_short!("ADMIN");
const PAUSED: Symbol = symbol_short!("PAUSED");

/// Role-based access control
#[contracttype]
#[derive(Clone, PartialEq)]
pub enum Role {
    Admin,
    Moderator,
    User,
}

/// Storage keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Role(Address),
    Balance(Address),
    Allowlist(Address),
}

#[contract]
pub struct AuthContract;

#[contractimpl]
impl AuthContract {
    /// Initialize with an admin
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }
        
        admin.require_auth();
        
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&PAUSED, &false);
        
        // Set admin role
        env.storage()
            .persistent()
            .set(&DataKey::Role(admin), &Role::Admin);
    }

    /// Simple auth: caller must authenticate
    pub fn set_balance(env: Env, user: Address, amount: i128) {
        user.require_auth();
        
        env.storage()
            .persistent()
            .set(&DataKey::Balance(user), &amount);
    }

    /// Admin-only function
    pub fn admin_set_balance(env: Env, admin: Address, user: Address, amount: i128) {
        admin.require_auth();
        
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .unwrap_or_else(|| panic!("not initialized"));
        
        if stored_admin != admin {
            panic!("not the admin");
        }
        
        env.storage()
            .persistent()
            .set(&DataKey::Balance(user), &amount);
    }

    /// Role-based auth: requires moderator or admin
    pub fn moderate_set_balance(env: Env, caller: Address, user: Address, amount: i128) {
        caller.require_auth();
        
        let role: Role = env
            .storage()
            .persistent()
            .get(&DataKey::Role(caller))
            .unwrap_or(Role::User);
        
        if role != Role::Admin && role != Role::Moderator {
            panic!("insufficient permissions");
        }
        
        env.storage()
            .persistent()
            .set(&DataKey::Balance(user), &amount);
    }

    /// Multi-auth: requires both admin and user
    pub fn multi_auth_transfer(
        env: Env,
        admin: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) {
        admin.require_auth();
        from.require_auth();
        
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .unwrap_or_else(|| panic!("not initialized"));
        
        if stored_admin != admin {
            panic!("not the admin");
        }
        
        let from_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);
        
        if from_balance < amount {
            panic!("insufficient balance");
        }
        
        let to_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);
        
        env.storage()
            .persistent()
            .set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to), &(to_balance + amount));
    }

    /// Pausable function
    pub fn pausable_operation(env: Env, user: Address) {
        user.require_auth();
        
        let paused: bool = env.storage().instance().get(&PAUSED).unwrap_or(false);
        
        if paused {
            panic!("contract is paused");
        }
        
        // Operation logic here
        let balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(user.clone()))
            .unwrap_or(0);
        
        env.storage()
            .persistent()
            .set(&DataKey::Balance(user), &(balance + 1));
    }

    /// Admin function to pause contract
    pub fn pause(env: Env, admin: Address) {
        admin.require_auth();
        
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .unwrap_or_else(|| panic!("not initialized"));
        
        if stored_admin != admin {
            panic!("not the admin");
        }
        
        env.storage().instance().set(&PAUSED, &true);
    }

    /// Admin function to unpause contract
    pub fn unpause(env: Env, admin: Address) {
        admin.require_auth();
        
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .unwrap_or_else(|| panic!("not initialized"));
        
        if stored_admin != admin {
            panic!("not the admin");
        }
        
        env.storage().instance().set(&PAUSED, &false);
    }

    /// Grant a role to an address
    pub fn grant_role(env: Env, admin: Address, user: Address, role: Role) {
        admin.require_auth();
        
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .unwrap_or_else(|| panic!("not initialized"));
        
        if stored_admin != admin {
            panic!("not the admin");
        }
        
        env.storage()
            .persistent()
            .set(&DataKey::Role(user), &role);
    }

    /// Get balance
    pub fn get_balance(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(user))
            .unwrap_or(0)
    }

    /// Get role
    pub fn get_role(env: Env, user: Address) -> Role {
        env.storage()
            .persistent()
            .get(&DataKey::Role(user))
            .unwrap_or(Role::User)
    }

    /// Check if paused
    pub fn is_paused(env: Env) -> bool {
        env.storage().instance().get(&PAUSED).unwrap_or(false)
    }

    /// Get admin address
    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&ADMIN)
            .unwrap_or_else(|| panic!("not initialized"))
    }
}

