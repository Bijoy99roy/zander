use std::slice::Iter;

use crate::{error::ErrorCode, News, Treasury, VoteRecord, Votes};
use crate::{
    NewsPhase, Verifier, BASE_SLASH_RATE_BP, BASE_SLASH_RATE_RP, BASIS_POINT, FEES_NUMERATOR,
    GAP_MULTIPLIER, MIN_VOTES_REQUIRED, SUPERMAJORITY,
};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_lang::AccountDeserialize;

#[derive(Accounts)]
pub struct FinalizeNews<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
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
    pub fn finalize(&mut self, remaining_accounts: &[AccountInfo<'info>]) -> Result<()> {
        let news = &mut self.news;

        require!(
            Clock::get()?.unix_timestamp > news.deadline,
            ErrorCode::VotingPhaseStillActive
        );
        require!(!news.finalized, ErrorCode::AlreadyFinalized);

        let mut total_voting_power = 0;
        let mut true_power = 0;
        let mut false_power = 0;

        let mut true_votes = 0;
        let mut false_votes = 0;

        let remaing_account_iter = remaining_accounts;

        for acc in remaing_account_iter.chunks_exact(3) {
            let mut data: &[u8] = &acc[0].try_borrow_data()?;
            let votes: VoteRecord = VoteRecord::try_deserialize(&mut data)?;
            total_voting_power += votes.voting_power;
            match votes.vote {
                Votes::True => {
                    true_power += votes.voting_power;
                    true_votes += 1;
                }
                Votes::False => {
                    false_power += votes.voting_power;
                    false_votes += 1;
                }
            }
        }

        let total_vp = true_power + false_power;
        let total_votes = true_votes + false_votes;

        // Check total vote count satisfies minimum required vote
        require!(total_votes >= MIN_VOTES_REQUIRED, ErrorCode::NotEnoughVotes);

        let perct = true_power.max(false_power) * 100 / total_vp;

        let mut total_winner = 0;
        let final_result = if true_power > false_power {
            total_winner = true_votes;
            Votes::True
        } else {
            total_winner = false_votes;
            Votes::False
        };

        if perct >= SUPERMAJORITY {
            news.finalized = true;
            news.phase = NewsPhase::Finalized;
            news.winner = Some(final_result.clone());
            news.vote_false = false_votes;
            news.vote_true = true_votes;
        } else {
            match news.phase {
                NewsPhase::Verification => {
                    let extended_deadline: i64 = 5; // 5 second
                    news.phase = NewsPhase::Dispute;
                    news.deadline = news
                        .deadline
                        .checked_add(extended_deadline)
                        .ok_or(ErrorCode::MathOverflow)?;
                }
                NewsPhase::Dispute => {
                    news.finalized = true;
                    news.phase = NewsPhase::Finalized;
                    news.winner = None;
                }
                _ => {}
            }
        }

        let gap = ((true_power.max(false_power) - false_power.min(true_power)) * 100) / total_vp;
        slash_and_reward(
            &self.treasury,
            remaining_accounts,
            final_result,
            gap,
            total_winner,
            &self.system_program,
        )?;

        Ok(())
    }
}

fn slash_and_reward<'info>(
    treasury: &Account<'info, Treasury>,
    remaining_accounts: &[AccountInfo<'info>],
    winner: Votes,
    gap: u64,
    total_winner: u64,
    system_program: &Program<'info, System>,
) -> Result<()> {
    let mut total_winner_power: u64 = 0;
    let mut total_slashed: u64 = 0;
    // Calculate total winning voting power

    for acc in remaining_accounts.chunks_exact(3) {
        if acc[0].owner != &crate::ID {
            continue;
        }

        let mut data: &[u8] = &acc[0].try_borrow_data()?;
        let votes: VoteRecord = VoteRecord::try_deserialize(&mut data)?;

        if votes.vote == winner {
            total_winner_power += votes.voting_power;
        }
    }

    // Slash incorrect voters stakes and reputation
    for acc in remaining_accounts.chunks_exact(3) {
        if acc[0].owner != &crate::ID {
            continue;
        }

        let mut data: &[u8] = &acc[0].try_borrow_data()?;
        let votes: VoteRecord = VoteRecord::try_deserialize(&mut data)?;

        if votes.vote == winner {
            continue;
        }

        // let verifier_data = remaining_accounts
        //     .find(|v| v.key() == votes.verifier)
        //     .ok_or(ErrorCode::MissingVerifier)?;

        let mut verifier = {
            let mut data: &[u8] = &acc[2].try_borrow_data()?;
            Verifier::try_deserialize(&mut data)?
        };

        // slash_rate = BASE × (1 + 4*gap)
        // This formula makes sure the slashing rate is proportional to the gap
        // Even if gap if 0, It doesn't make the rate zero
        let slash_rate_bp = BASE_SLASH_RATE_BP
            .checked_mul(BASIS_POINT + GAP_MULTIPLIER * gap)
            .ok_or(ErrorCode::MathOverflow)?
            / BASIS_POINT;

        let slash_amount = verifier
            .stake_lamports
            .checked_mul(slash_rate_bp)
            .ok_or(ErrorCode::MathOverflow)?
            / BASIS_POINT;

        verifier.stake_lamports = verifier.stake_lamports.saturating_sub(slash_amount);

        total_slashed = total_slashed
            .checked_add(slash_amount)
            .ok_or(ErrorCode::MathOverflow)?;

        // let (expected_vault, _) = Pubkey::find_program_address(
        //     &[b"stake_vault", verifier.verifier.key().as_ref()],
        //     &system_program.key(),
        // );

        // let vault = remaining_accounts
        //     .find(|a| a.key == &expected_vault)
        //     .ok_or(ErrorCode::VaultNotFound)?;

        let vault = &acc[1];

        // **vault.try_borrow_mut_lamports()? -= slash_amount;

        let verifier_key = verifier.verifier.key();
        let seeds = &[
            b"stake_vault",
            verifier_key.as_ref(),
            &[verifier.vault_bump],
        ];

        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            system_program.to_account_info(),
            Transfer {
                from: vault.to_account_info(),
                to: treasury.to_account_info(),
            },
            signer,
        );

        transfer(cpi_ctx, slash_amount)?;

        let reputation = decrease_reputation(verifier.reputation, gap);
        verifier.reputation = verifier.reputation.saturating_sub(reputation);

        let mut data: &mut [u8] = &mut acc[2].try_borrow_mut_data()?;
        verifier.try_serialize(&mut data)?;
    }

    let treasury_cut = total_slashed
        .checked_mul(FEES_NUMERATOR)
        .ok_or(ErrorCode::MathOverflow)?
        / BASIS_POINT;

    // **treasury.to_account_info().try_borrow_mut_lamports()? += treasury_cut;

    let reward_pool = total_slashed.saturating_sub(treasury_cut);

    let mut winner_left = total_winner;
    let mut remaining_rewards = reward_pool;
    // reward correct voters stakes and reputation
    for acc in remaining_accounts.chunks_exact(3) {
        if acc[0].owner != &crate::ID {
            continue;
        }

        let mut data: &[u8] = &acc[0].try_borrow_data()?;
        let votes: VoteRecord = VoteRecord::try_deserialize(&mut data)?;

        if votes.vote != winner {
            continue;
        }

        // let verifier_data = remaining_accounts
        //     .find(|v| v.key() == votes.verifier)
        //     .ok_or(ErrorCode::MissingVerifier)?;

        let mut verifier = {
            let mut data: &[u8] = &acc[2].try_borrow_data()?;
            Verifier::try_deserialize(&mut data)?
        };

        let reward = if winner_left == 1 {
            remaining_rewards
        } else {
            let r = reward_pool
                .checked_mul(votes.voting_power)
                .ok_or(ErrorCode::MathOverflow)?
                / total_winner_power;
            remaining_rewards = remaining_rewards
                .checked_sub(r)
                .ok_or(ErrorCode::MathOverflow)?;
            r
        };
        winner_left -= 1;
        verifier.stake_lamports = verifier
            .stake_lamports
            .checked_add(reward as u64)
            .ok_or(ErrorCode::MathOverflow)?;

        // let (expected_vault, _) = Pubkey::find_program_address(
        //     &[b"stake_vault", verifier.verifier.key().as_ref()],
        //     &system_program.key(),
        // );

        // let vault = remaining_accounts
        //     .find(|a| a.key == &expected_vault)
        //     .ok_or(ErrorCode::VaultNotFound)?;

        let vault = &acc[1];

        **treasury.to_account_info().try_borrow_mut_lamports()? -= reward;
        **vault.try_borrow_mut_lamports()? += reward;

        let reputation = increase_reputaton(verifier.reputation, gap);
        verifier.reputation = verifier.reputation.saturating_add(reputation).min(100);

        let mut data: &mut [u8] = &mut acc[2].try_borrow_mut_data()?;
        verifier.try_serialize(&mut data)?;
    }
    Ok(())
}

fn increase_reputaton(reputation: u64, gap: u64) -> u64 {
    // Headroom makes sure reputation increases more when initial reputation is low and reputation increases less when initial reputation is high
    let headroom = 100u64.saturating_sub(reputation);

    let rep_gain = (headroom * (1 + gap)) / 100;

    rep_gain.max(1)
}

fn decrease_reputation(reputation: u64, gap: u64) -> u64 {
    let base_loss = reputation * BASE_SLASH_RATE_RP;

    let gap_influence = (100 + GAP_MULTIPLIER * gap) / 100;

    let rep_loss = (base_loss * gap_influence) / BASIS_POINT;

    rep_loss.max(1)
}
