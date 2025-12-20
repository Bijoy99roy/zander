use anchor_lang::prelude::*;

use crate::Treasury;

#[derive(Accounts)]
pub struct InitTreasury<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + Treasury::INIT_SPACE,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Account<'info, Treasury>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitTreasury<'info> {
    pub fn init(&mut self, bump: u8) -> Result<()> {
        self.treasury.bump = bump;
        Ok(())
    }
}
