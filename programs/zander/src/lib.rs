use anchor_lang::prelude::*;
pub mod state;
pub use state::*;
pub mod instructions;
pub use instructions::*;

declare_id!("77thxrK3p7t7SBr1Wk3VyvhMbDu94UDfnqVFHXWZAdh6");

#[program]
pub mod zander {
    use super::*;

    pub fn post_news(ctx: Context<PostNews>, ipfs_url: String, headline: String) -> Result<()> {
        let bump = ctx.bumps.news;
        ctx.accounts.post(ipfs_url, headline, bump)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
