use anchor_lang::{
    prelude::*,
    solana_program::{hash::hash, native_token::LAMPORTS_PER_SOL},
};

use crate::{error::ErrorCode, News, Verifier, VoteRecord, Votes, MIN_STAKE_SOL, TOKENS_PER_SOL};
use integer_sqrt::IntegerSquareRoot;

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(
        mut,
        seeds = [b"verifier", voter.key().as_ref()],
        bump=verifier.bump,
    )]
    pub verifier: Account<'info, Verifier>,

    #[account(
        mut,
        seeds = [b"news", news.publisher.key().as_ref(), &hash(news.ipfs_url.as_bytes()).to_bytes()[..]],
        bump=news.bump
    )]
    pub news: Account<'info, News>,

    #[account(
        init,
        payer = voter,
        space = 8 + VoteRecord::INIT_SPACE,
        seeds = [b"vote", news.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>,

    pub system_program: Program<'info, System>,
}

impl<'info> CastVote<'info> {
    pub fn vote(&mut self, vote: Votes) -> Result<()> {
        let verifier = &self.verifier;
        require!(
            verifier.stake_lamports >= MIN_STAKE_SOL,
            ErrorCode::NotVerifier
        );

        let vote_record = &mut self.vote_record;

        vote_record.verifier = self.voter.key();
        vote_record.vote = vote;
        let voting_power =
            calc_voting_power(self.verifier.stake_lamports, self.verifier.reputation);
        vote_record.voting_power = voting_power;

        Ok(())
    }
}

fn calc_voting_power(stake: u64, rep: u8) -> u64 {
    /*
       VPT_TOKENS = SOL_STAKED * TOKENS_PER_SOL
       SCALED_POWER = SQRT(VPT_TOKENS)
       VOTING_POWER = SCALED_POWER * (100 + REPUTATION_POINT) / 100
    */
    let tokens = stake / LAMPORTS_PER_SOL * TOKENS_PER_SOL;
    let sqrt_tokens = tokens.integer_sqrt();
    sqrt_tokens * (100 + rep as u64) / 100
}
