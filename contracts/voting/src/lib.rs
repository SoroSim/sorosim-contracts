#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol};

const PROPOSAL_COUNTER: Symbol = symbol_short!("COUNTER");

/// Proposal data structure
#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u64,
    pub description: String,
    pub creator: Address,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub deadline: u64,
}

/// Storage keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Proposal(u64),
    Vote(u64, Address), // (proposal_id, voter)
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    /// Create a new proposal
    pub fn create_proposal(
        env: Env,
        creator: Address,
        description: String,
        deadline: u64,
    ) -> u64 {
        creator.require_auth();
        
        // Get next proposal ID
        let proposal_id: u64 = env
            .storage()
            .instance()
            .get(&PROPOSAL_COUNTER)
            .unwrap_or(0);
        
        // Create proposal
        let proposal = Proposal {
            id: proposal_id,
            description,
            creator,
            yes_votes: 0,
            no_votes: 0,
            deadline,
        };
        
        // Store proposal
        env.storage()
            .persistent()
            .set(&DataKey::Proposal(proposal_id), &proposal);
        
        // Increment counter
        env.storage()
            .instance()
            .set(&PROPOSAL_COUNTER, &(proposal_id + 1));
        
        proposal_id
    }

    /// Cast a vote on a proposal
    pub fn vote(env: Env, proposal_id: u64, voter: Address, in_favor: bool) {
        voter.require_auth();
        
        // Check if already voted
        let vote_key = DataKey::Vote(proposal_id, voter.clone());
        if env.storage().persistent().has(&vote_key) {
            panic!("already voted");
        }
        
        // Get proposal
        let mut proposal: Proposal = env
            .storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap_or_else(|| panic!("proposal does not exist"));
        
        // Check deadline
        if env.ledger().timestamp() > proposal.deadline {
            panic!("voting period ended");
        }
        
        // Update vote counts
        if in_favor {
            proposal.yes_votes += 1;
        } else {
            proposal.no_votes += 1;
        }
        
        // Store updated proposal
        env.storage()
            .persistent()
            .set(&DataKey::Proposal(proposal_id), &proposal);
        
        // Record that voter has voted
        env.storage().persistent().set(&vote_key, &in_favor);
    }

    /// Get proposal details
    pub fn get_proposal(env: Env, proposal_id: u64) -> Proposal {
        env.storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap_or_else(|| panic!("proposal does not exist"))
    }

    /// Check if an address has voted on a proposal
    pub fn has_voted(env: Env, proposal_id: u64, voter: Address) -> bool {
        let vote_key = DataKey::Vote(proposal_id, voter);
        env.storage().persistent().has(&vote_key)
    }

    /// Get the total number of proposals
    pub fn total_proposals(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&PROPOSAL_COUNTER)
            .unwrap_or(0)
    }
}

