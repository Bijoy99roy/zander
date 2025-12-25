use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};

pub const MIN_STAKE_SOL: u64 = 5 * LAMPORTS_PER_SOL;
pub const TOKENS_PER_SOL: u64 = 10;

pub const FEES_NUMERATOR: u64 = 500;
pub const BASIS_POINT: u64 = 10000;

pub const SUPERMAJORITY: u64 = 66;

pub const MIN_VOTES_REQUIRED: u64 = 20;

pub const BASE_SLASH_RATE_BP: u64 = 200;

pub const GAP_MULTIPLIER: u64 = 4;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum Votes {
    True,
    False,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum NewsPhase {
    Verification,
    Dispute,
    Finalized,
}

#[account]
#[derive(InitSpace)]
pub struct News {
    pub publisher: Pubkey,
    pub created_at: i64,
    pub vote_true: u64,
    pub vote_false: u64,
    pub finalized: bool,
    pub bump: u8,
    pub phase: NewsPhase,
    pub winner: Option<Votes>,
    #[max_len(200)]
    pub headline: String,
    #[max_len(300)]
    pub ipfs_url: String,
}

#[account]
#[derive(InitSpace)]
pub struct VoteRecord {
    pub verifier: Pubkey,
    pub vote: Votes,
    pub voting_power: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Verifier {
    pub verifier: Pubkey,
    pub reputation: u8,
    pub stake_lamports: u64,
    pub bump: u8,
    pub vault_bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    pub bump: u8,
}
