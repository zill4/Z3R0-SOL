use anchor_lang::prelude::*;

declare_id!("3kVGkkFSJZEpPhiDuGBXbYRC3jLz66EffCJeEWy7NPbQ");

#[program]
pub mod z_sol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
