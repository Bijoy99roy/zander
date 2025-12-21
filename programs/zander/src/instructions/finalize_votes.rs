use std::slice::Iter;

use crate::{error::ErrorCode, News, Treasury, VoteRecord, Votes};
use crate::{NewsPhase, SUPERMAJORITY};
use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;

#[derive(Accounts)]
pub struct FinalizeNews<'info> {
    #[account(mut)]
    pub news: Account<'info, News>,

    #[account(
        mut,
        seeds = [b"treasury"],
        bump=treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,

    pub system_program: Program<'info, System>,
}

impl<'info> FinalizeNews<'info> {
    pub fn finalize(
        &mut self,
        remaining_accounts: std::slice::Iter<'_, AccountInfo<'_>>,
    ) -> Result<()> {
        let news = &mut self.news;
        require!(!news.finalized, ErrorCode::AlreadyFinalized);
        let mut total_voting_power = 0;
        let mut true_power = 0;
        let mut false_power = 0;

        let remaing_account_iter = remaining_accounts;

        for acc in remaing_account_iter {
            let mut data: &[u8] = &acc.try_borrow_data()?;
            let votes: VoteRecord = VoteRecord::try_deserialize(&mut data)?;
            total_voting_power += votes.voting_power;
            match votes.vote {
                Votes::True => true_power += votes.voting_power,
                Votes::False => false_power += votes.voting_power,
            }
        }

        let total_vp = true_power + false_power;
        let perct = true_power.max(false_power) * 100 / total_vp;
        let final_result = if true_power > false_power {
            Votes::True
        } else {
            Votes::False
        };
        if perct >= SUPERMAJORITY {
            news.finalized = true;
            news.phase = NewsPhase::Finalized;
            news.winner = Some(final_result);
        } else {
            match news.phase {
                NewsPhase::Verification => {
                    news.phase = NewsPhase::Dispute;
                }
                NewsPhase::Dispute => {
                    news.finalized = true;
                    news.phase = NewsPhase::Finalized;
                    news.winner = None;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
