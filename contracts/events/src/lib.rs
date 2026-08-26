#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, String, Vec};

/// Event topics (symbols)
const TRANSFER: Symbol = symbol_short!("TRANSFER");
const APPROVAL: Symbol = symbol_short!("APPROVAL");
const STATUS: Symbol = symbol_short!("STATUS");
const MULTI: Symbol = symbol_short!("MULTI");

/// Complex event data structure
#[contracttype]
#[derive(Clone)]
pub struct TransferData {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
}

/// Status enum for event emission
#[contracttype]
#[derive(Clone, PartialEq)]
pub enum Status {
    Active,
    Paused,
    Stopped,
}

#[contract]
pub struct EventsContract;

#[contractimpl]
impl EventsContract {
    /// Emit a simple event with single topic and no data
    pub fn emit_simple(env: Env) {
        env.events().publish((symbol_short!("SIMPLE"),), ());
    }

    /// Emit an event with a single value
    pub fn emit_with_value(env: Env, value: i128) {
        env.events().publish((symbol_short!("VALUE"),), value);
    }

    /// Emit an event with an address
    pub fn emit_with_address(env: Env, address: Address) {
        env.events().publish((symbol_short!("ADDRESS"),), address);
    }

    /// Emit a transfer event (two addresses and amount)
    pub fn emit_transfer(env: Env, from: Address, to: Address, amount: i128) {
        env.events().publish(
            (TRANSFER, from.clone(), to.clone()),
            amount,
        );
    }

    /// Emit an approval event with structured data
    pub fn emit_approval(env: Env, owner: Address, spender: Address, amount: i128) {
        env.events().publish(
            (APPROVAL, owner, spender),
            amount,
        );
    }

    /// Emit an event with a complex data structure
    pub fn emit_transfer_data(env: Env, from: Address, to: Address, amount: i128) {
        let data = TransferData {
            from: from.clone(),
            to: to.clone(),
            amount,
        };
        env.events().publish((TRANSFER,), data);
    }

    /// Emit an event with enum data
    pub fn emit_status_change(env: Env, old_status: Status, new_status: Status) {
        env.events().publish(
            (STATUS,),
            (old_status, new_status),
        );
    }

    /// Emit an event with string data
    pub fn emit_with_string(env: Env, message: String) {
        env.events().publish(
            (symbol_short!("MESSAGE"),),
            message,
        );
    }

    /// Emit an event with tuple data
    pub fn emit_with_tuple(env: Env, user: Address, value1: i128, value2: i128) {
        env.events().publish(
            (symbol_short!("TUPLE"),),
            (user, value1, value2),
        );
    }

    /// Emit an event with vector data
    pub fn emit_with_vec(env: Env, values: Vec<i128>) {
        env.events().publish(
            (symbol_short!("VECTOR"),),
            values,
        );
    }

    /// Emit multiple events in one call
    pub fn emit_multiple(env: Env, user: Address, amount: i128) {
        // First event: operation started
        env.events().publish(
            (symbol_short!("START"),),
            user.clone(),
        );
        
        // Second event: value processed
        env.events().publish(
            (symbol_short!("PROCESS"),),
            amount,
        );
        
        // Third event: operation completed
        env.events().publish(
            (symbol_short!("COMPLETE"), user),
            amount,
        );
    }

    /// Emit event with multiple topics
    pub fn emit_multi_topic(env: Env, topic1: Symbol, topic2: Symbol, value: i128) {
        env.events().publish(
            (MULTI, topic1, topic2),
            value,
        );
    }

    /// Emit event with boolean data
    pub fn emit_with_bool(env: Env, flag: bool) {
        env.events().publish(
            (symbol_short!("FLAG"),),
            flag,
        );
    }

    /// Emit event with u64 data
    pub fn emit_with_u64(env: Env, timestamp: u64) {
        env.events().publish(
            (symbol_short!("TIME"),),
            timestamp,
        );
    }

    /// Emit event with u32 data
    pub fn emit_with_u32(env: Env, count: u32) {
        env.events().publish(
            (symbol_short!("COUNT"),),
            count,
        );
    }

    /// Demonstrate event emission in a typical contract operation
    pub fn transfer_with_events(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) {
        from.require_auth();
        
        // Emit event: transfer initiated
        env.events().publish(
            (symbol_short!("TX_START"),),
            (from.clone(), to.clone(), amount),
        );
        
        // Business logic would go here
        
        // Emit event: transfer completed
        env.events().publish(
            (TRANSFER, from, to),
            amount,
        );
    }
}

