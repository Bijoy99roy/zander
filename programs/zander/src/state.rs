use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};

pub const MIN_STAKE_SOL: u64 = 5 * LAMPORTS_PER_SOL;

pub const FEES_NUMERATOR: u64 = 500;
pub const FEES_DENOMINATOR: u64 = 10000;

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
    pub created_at: i64,
    pub vote_true: u128,
    pub vote_false: u128,
    pub finalized: bool,
    pub bump: u8,
    pub phase: NewsPhase,
    pub winner: Option<VoteRecord>,
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
    pub voting_power: u128,
}

#[account]
#[derive(InitSpace)]
pub struct Verifier {
    pub verifier: Pubkey,
    pub reputation: u64,
    pub stake_lamports: u64,
    pub voting_power: u64,
    pub bump: u8,
    pub vault_bump: u8,
}
