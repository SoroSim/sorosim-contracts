#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol};

const PROPOSAL_COUNTER: Symbol = symbol_short!("COUNTER");

/// Proposal status after finalization
#[contracttype]
#[derive(Clone, PartialEq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Tied,
}

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
    pub status: ProposalStatus,
    pub finalized: bool,
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
            status: ProposalStatus::Active,
            finalized: false,
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
        
        // Check if finalized
        if proposal.finalized {
            panic!("proposal already finalized");
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

    /// Finalize a proposal after the voting period ends
    pub fn finalize(env: Env, proposal_id: u64) {
        // Get proposal
        let mut proposal: Proposal = env
            .storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap_or_else(|| panic!("proposal does not exist"));
        
        // Check if already finalized
        if proposal.finalized {
            panic!("proposal already finalized");
        }
        
        // Check if voting period has ended
        if env.ledger().timestamp() <= proposal.deadline {
            panic!("voting period not ended");
        }
        
        // Determine outcome
        proposal.status = if proposal.yes_votes > proposal.no_votes {
            ProposalStatus::Passed
        } else if proposal.no_votes > proposal.yes_votes {
            ProposalStatus::Rejected
        } else {
            ProposalStatus::Tied
        };
        
        proposal.finalized = true;
        
        // Store updated proposal
        env.storage()
            .persistent()
            .set(&DataKey::Proposal(proposal_id), &proposal);
    }

    /// Get the current status of a proposal
    pub fn get_status(env: Env, proposal_id: u64) -> ProposalStatus {
        let proposal: Proposal = env
            .storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap_or_else(|| panic!("proposal does not exist"));
        
        proposal.status
    }

    /// Get vote tally for a proposal
    pub fn get_tally(env: Env, proposal_id: u64) -> (u64, u64) {
        let proposal: Proposal = env
            .storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap_or_else(|| panic!("proposal does not exist"));
        
        (proposal.yes_votes, proposal.no_votes)
    }

    /// Check if a proposal is finalized
    pub fn is_finalized(env: Env, proposal_id: u64) -> bool {
        let proposal: Proposal = env
            .storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .unwrap_or_else(|| panic!("proposal does not exist"));
        
        proposal.finalized
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_create_proposal() {
        let env = Env::default();
        let contract_id = env.register_contract(None, VotingContract);
        let client = VotingContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        env.mock_all_auths();

        let deadline = env.ledger().timestamp() + 1000;
        let proposal_id = client.create_proposal(&creator, &String::from_str(&env, "Test"), &deadline);

        assert_eq!(proposal_id, 0);
        assert_eq!(client.total_proposals(), 1);

        let proposal = client.get_proposal(&proposal_id);
        assert_eq!(proposal.id, 0);
        assert_eq!(proposal.yes_votes, 0);
        assert_eq!(proposal.no_votes, 0);
        assert_eq!(proposal.finalized, false);
    }

    #[test]
    fn test_vote_yes() {
        let env = Env::default();
        let contract_id = env.register_contract(None, VotingContract);
        let client = VotingContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let voter = Address::generate(&env);
        env.mock_all_auths();

        let deadline = env.ledger().timestamp() + 1000;
        let proposal_id = client.create_proposal(&creator, &String::from_str(&env, "Test"), &deadline);

        client.vote(&proposal_id, &voter, &true);

        let (yes, no) = client.get_tally(&proposal_id);
        assert_eq!(yes, 1);
        assert_eq!(no, 0);
    }

    #[test]
    fn test_vote_no() {
        let env = Env::default();
        let contract_id = env.register_contract(None, VotingContract);
        let client = VotingContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let voter = Address::generate(&env);
        env.mock_all_auths();

        let deadline = env.ledger().timestamp() + 1000;
        let proposal_id = client.create_proposal(&creator, &String::from_str(&env, "Test"), &deadline);

        client.vote(&proposal_id, &voter, &false);

        let (yes, no) = client.get_tally(&proposal_id);
        assert_eq!(yes, 0);
        assert_eq!(no, 1);
    }

    #[test]
    fn test_multiple_votes() {
        let env = Env::default();
        let contract_id = env.register_contract(None, VotingContract);
        let client = VotingContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        env.mock_all_auths();

        let deadline = env.ledger().timestamp() + 1000;
        let proposal_id = client.create_proposal(&creator, &String::from_str(&env, "Test"), &deadline);

        // Multiple voters
        for _ in 0..3 {
            let voter = Address::generate(&env);
            client.vote(&proposal_id, &voter, &true);
        }

        for _ in 0..2 {
            let voter = Address::generate(&env);
            client.vote(&proposal_id, &voter, &false);
        }

        let (yes, no) = client.get_tally(&proposal_id);
        assert_eq!(yes, 3);
        assert_eq!(no, 2);
    }

    #[test]
    #[should_panic(expected = "already voted")]
    fn test_double_vote() {
        let env = Env::default();
        let contract_id = env.register_contract(None, VotingContract);
        let client = VotingContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let voter = Address::generate(&env);
        env.mock_all_auths();

        let deadline = env.ledger().timestamp() + 1000;
        let proposal_id = client.create_proposal(&creator, &String::from_str(&env, "Test"), &deadline);

        client.vote(&proposal_id, &voter, &true);
        client.vote(&proposal_id, &voter, &false); // Should panic
    }

    #[test]
    fn test_has_voted() {
        let env = Env::default();
        let contract_id = env.register_contract(None, VotingContract);
        let client = VotingContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let voter = Address::generate(&env);
        let non_voter = Address::generate(&env);
        env.mock_all_auths();

        let deadline = env.ledger().timestamp() + 1000;
        let proposal_id = client.create_proposal(&creator, &String::from_str(&env, "Test"), &deadline);

        assert_eq!(client.has_voted(&proposal_id, &voter), false);

        client.vote(&proposal_id, &voter, &true);

        assert_eq!(client.has_voted(&proposal_id, &voter), true);
        assert_eq!(client.has_voted(&proposal_id, &non_voter), false);
    }

    #[test]
    fn test_finalize_passed() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);
        
        let contract_id = env.register_contract(None, VotingContract);
        let client = VotingContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        env.mock_all_auths();

        let deadline = 200;
        let proposal_id = client.create_proposal(&creator, &String::from_str(&env, "Test"), &deadline);

        // Vote yes > no
        for _ in 0..3 {
            let voter = Address::generate(&env);
            client.vote(&proposal_id, &voter, &true);
        }
        for _ in 0..1 {
            let voter = Address::generate(&env);
            client.vote(&proposal_id, &voter, &false);
        }

        // Move time forward
        env.ledger().with_mut(|li| li.timestamp = 201);

        client.finalize(&proposal_id);

        assert_eq!(client.is_finalized(&proposal_id), true);
        assert_eq!(client.get_status(&proposal_id), ProposalStatus::Passed);
    }

    #[test]
    fn test_finalize_rejected() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);
        
        let contract_id = env.register_contract(None, VotingContract);
        let client = VotingContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        env.mock_all_auths();

        let deadline = 200;
        let proposal_id = client.create_proposal(&creator, &String::from_str(&env, "Test"), &deadline);

        // Vote no > yes
        for _ in 0..1 {
            let voter = Address::generate(&env);
            client.vote(&proposal_id, &voter, &true);
        }
        for _ in 0..3 {
            let voter = Address::generate(&env);
            client.vote(&proposal_id, &voter, &false);
        }

        env.ledger().with_mut(|li| li.timestamp = 201);
        client.finalize(&proposal_id);

        assert_eq!(client.get_status(&proposal_id), ProposalStatus::Rejected);
    }

    #[test]
    fn test_finalize_tied() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);
        
        let contract_id = env.register_contract(None, VotingContract);
        let client = VotingContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        env.mock_all_auths();

        let deadline = 200;
        let proposal_id = client.create_proposal(&creator, &String::from_str(&env, "Test"), &deadline);

        // Equal votes
        for _ in 0..2 {
            let voter = Address::generate(&env);
            client.vote(&proposal_id, &voter, &true);
        }
        for _ in 0..2 {
            let voter = Address::generate(&env);
            client.vote(&proposal_id, &voter, &false);
        }

        env.ledger().with_mut(|li| li.timestamp = 201);
        client.finalize(&proposal_id);

        assert_eq!(client.get_status(&proposal_id), ProposalStatus::Tied);
    }

    #[test]
    #[should_panic(expected = "voting period not ended")]
    fn test_finalize_before_deadline() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);
        
        let contract_id = env.register_contract(None, VotingContract);
        let client = VotingContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        env.mock_all_auths();

        let deadline = 200;
        let proposal_id = client.create_proposal(&creator, &String::from_str(&env, "Test"), &deadline);

        client.finalize(&proposal_id); // Should panic
    }

    #[test]
    #[should_panic(expected = "proposal already finalized")]
    fn test_vote_after_finalize() {
        let env = Env::default();
        env.ledger().with_mut(|li| li.timestamp = 100);
        
        let contract_id = env.register_contract(None, VotingContract);
        let client = VotingContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let voter = Address::generate(&env);
        env.mock_all_auths();

        let deadline = 200;
        let proposal_id = client.create_proposal(&creator, &String::from_str(&env, "Test"), &deadline);

        env.ledger().with_mut(|li| li.timestamp = 201);
        client.finalize(&proposal_id);

        client.vote(&proposal_id, &voter, &true); // Should panic
    }
}
