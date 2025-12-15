use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};

pub const STAKE_SOL: u64 = 5 * LAMPORTS_PER_SOL;

enum Votes {
    True,
    False,
}

struct VoteCount {
    pub true_count: u64,
    pub false_count: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
struct Fees {
    pub fee_numerator: u64,
    pub fee_denominator: u64,
}

#[account]
pub struct News {}
