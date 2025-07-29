use anchor_lang::prelude::*;

declare_id!("5YZMwaJFBUn3nwRocRYTm37qarZexjHd8pXmAbDrQjEg");

#[program]
pub mod branching_comics {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
