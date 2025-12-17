use anchor_lang::{prelude::*, solana_program::hash::hash};

use crate::{News, NewsPhase};

#[derive(Accounts)]
#[instruction(ipfs_url: String)]
pub struct PostNews<'info> {
    #[account(mut)]
    pub publisher: Signer<'info>,

    #[account(
        init,
        payer = publisher,
        space = 8 + News::INIT_SPACE,
        seeds = [b"news", publisher.key().as_ref(), &hash(ipfs_url.as_bytes()).to_bytes()[..]],
        bump
    )]
    pub news: Account<'info, News>,

    pub system_program: Program<'info, System>,
}

impl<'info> PostNews<'info> {
    pub fn post(&mut self, ipfs_url: String, headline: String, bump: u8) -> Result<()> {
        let news = &mut self.news;
        let clock = Clock::get()?;
        news.created_at = clock.unix_timestamp;
        news.publisher = self.publisher.key();
        news.vote_true = 0;
        news.vote_false = 0;
        news.finalized = false;
        news.phase = NewsPhase::Verification;
        news.winner = None;
        news.headline = headline;
        news.ipfs_url = ipfs_url;
        news.bump = bump;
        Ok(())
    }
}
