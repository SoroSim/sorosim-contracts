#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct CalleeContract;

#[contractimpl]
impl CalleeContract {}

