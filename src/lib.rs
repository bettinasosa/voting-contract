#![doc = include_str!("../README.md")]
#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate pbc_contract_common;

use pbc_contract_common::{Address, ContractContext};
use pbc_contract_codegen::{action, state};
use sorted_vec::{SortedVecMap, SortedVecSet};

#[state]
pub struct VoteState {
    pub proposal_id: u32,
    pub voters: SortedVecSet<Address>, // Eligible voters
    pub deadline_utc_millis: u64,      // Voting deadline
    pub votes: SortedVecMap<Address, bool>, // Voter address to their vote
    pub result: Option<bool>,          // Voting result after counting
    pub description: String,           // Proposal description
}

impl VoteState {
    pub fn is_voter_eligible(&self, address: &Address) -> bool {
        self.voters.contains(address)
    }
}


/// Initialize a new vote for a proposal
///
/// # Arguments
///
/// * `_ctx` - the contract context containing information about the sender and the blockchain.
/// * `proposal_id` - the id of the proposal.
/// * `voters` - the list of eligible voters.
/// * `deadline` - deadline of the vote in UTC millis.
/// * `description` - description of the vote
///
/// # Returns
///
/// The initial state of the vote.
///
#[init]
pub fn initialize(
  ctx: ContractContext, 
  proposal_id: u32, 
  voters: Vec<Address>, 
  deadline: u64, 
  description: String
) -> VoteState {
    // Ensure no duplicate or empty voters
    let unique_voters: SortedVecSet<Address> = voters.into_iter().collect();
    if unique_voters.is_empty() {
        panic!("Voter list cannot be empty.");
    }

    VoteState {
        proposal_id,
        voters: unique_voters,
        deadline_utc_millis: deadline,
        votes: SortedVecMap::new(),
        result: None,
        description,
    }
}


/// Cast a vote for the proposal.
/// The vote is cast by the sender of the action.
/// Voters can cast and update their vote until the deadline.
///
/// # Arguments
///
/// * `ctx` - the contract context containing information about the sender and the blockchain.
/// * `state` - the current state of the vote.
/// * `vote` - the vote being cast by the sender.
///
/// # Returns
///
/// The updated vote state reflecting the newly cast vote.

#[action(shortname = 0x01)]
pub fn vote(ctx: ContractContext, state: &mut VoteState, vote: bool) -> VoteState {
    let voter = ctx.sender;

    // Check if voter is eligible and within the deadline
    assert!(state.voters.contains(&voter), "You are not an eligible voter for this proposal.");
     assert!(
        state.result.is_none() && ctx.block_production_time < state.deadline_utc_millis,
        "Voting period has ended."
    );

    // Cast or update the vote
    state.votes.insert(voter, vote);
    state
}

/// Count the votes and publish the result.
/// Counting will fail if the deadline has not passed.
///
/// # Arguments
///
/// * `ctx` - the contract context containing information about the sender and blockchain.
/// * `state` - the current state of the vote.
///
/// # Returns
///
/// The updated state reflecting the result of the vote.
///
#[action(shortname = 0x02)]
pub fn count(ctx: ContractContext,state: &mut VoteState) -> VoteState {
    assert!(
        ctx.block_production_time >= state.deadline_utc_millis,
        "Voting is still ongoing."
    );
     // Prevent counting more than once
    if state.result.is_some() {
        panic!("Voting has already been counted.");
    }
    
     // Tally the votes
    let votes_for = state.votes.values().filter(|&&v| v).count();
    let total_votes = state.votes.len();
    let passed = votes_for > (total_votes / 2);
    
    state.result = Some(vote_passed);
    state
}