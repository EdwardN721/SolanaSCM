use anchor_lang::prelude::*;

declare_id!("A5i8uPKdCycDG3nbGCCAUiLzHEc4ddpfeYGQhPEWuaTJ");

#[program]
pub mod solana_scm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
