use anchor_lang::prelude::*;
pub mod state;
pub use state::*;
pub mod instructions;
pub use instructions::*;
pub mod error;
pub use error::*;

declare_id!("77thxrK3p7t7SBr1Wk3VyvhMbDu94UDfnqVFHXWZAdh6");

#[program]
pub mod zander {
    use super::*;

    pub fn initialize_treasury(ctx: Context<InitTreasury>) -> Result<()> {
        ctx.accounts.init(ctx.bumps.treasury)?;
        Ok(())
    }

    pub fn post_news(ctx: Context<PostNews>, ipfs_url: String, headline: String) -> Result<()> {
        let bump = ctx.bumps.news;
        ctx.accounts.post(ipfs_url, headline, bump)?;
        Ok(())
    }

    pub fn become_verifier(ctx: Context<BecomeVerifier>, stake_lamports: u64) -> Result<()> {
        let bump = ctx.bumps.verifier;
        let vault_bump = ctx.bumps.stake_vault;
        ctx.accounts
            .become_verifier(stake_lamports, bump, vault_bump)?;
        Ok(())
    }

    pub fn vote_news(ctx: Context<CastVote>, vote: Votes) -> Result<()> {
        ctx.accounts.vote(vote)?;
        Ok(())
    }

    pub fn finalize_news(ctx: Context<FinalizeNews>) -> Result<()> {
        let remaining_accounts = ctx.remaining_accounts.iter();
        ctx.accounts.finalize(remaining_accounts)?;
        Ok(())
    }
}
