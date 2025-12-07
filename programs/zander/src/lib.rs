use anchor_lang::prelude::*;

declare_id!("77thxrK3p7t7SBr1Wk3VyvhMbDu94UDfnqVFHXWZAdh6");

#[program]
pub mod zander {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
