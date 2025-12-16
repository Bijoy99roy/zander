use anchor_lang::{
    prelude::*,
    system_program::{self, Transfer},
};

use crate::{error::ErrorCode, Verifier, MIN_STAKE_SOL};

#[derive(Accounts)]
pub struct BecomeVerifier<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Verifier::INIT_SPACE,
        seeds = [b"verifier", user.key().as_ref()],
        bump
    )]
    pub verifier: Account<'info, Verifier>,

    /// CHECK: This PDA is derived inside progrm and doesn't need checking
    #[account(
        init,
        payer=user,
        owner=system_program::ID,
        space=0,
        seeds=[b"stake_vault", user.key().as_ref()],
        bump
    )]
    pub stake_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> BecomeVerifier<'info> {
    pub fn become_verifier(&mut self, stake_lamports: u64, bump: u8, vault_bump: u8) -> Result<()> {
        require!(
            stake_lamports >= MIN_STAKE_SOL,
            ErrorCode::InsufficientStake
        );

        let verifier = &mut self.verifier;
        verifier.verifier = self.user.key();
        verifier.stake_lamports = stake_lamports;
        verifier.reputation = 0;
        verifier.bump = bump;
        verifier.vault_bump = vault_bump;

        let ix = Transfer {
            from: self.user.to_account_info(),
            to: self.stake_vault.to_account_info(),
        };

        let cpi_context = CpiContext::new(self.system_program.to_account_info(), ix);

        system_program::transfer(cpi_context, stake_lamports)?;

        Ok(())
    }
}
