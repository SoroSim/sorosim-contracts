#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct CounterContract;

#[contractimpl]
impl CounterContract {}

