use anchor_lang::prelude::*;

use crate::Choice;


#[derive(Accounts)]
pub struct InitChoice<'info> {
  
  #[account(mut)]
  pub user: Signer<'info>,

  pub system_program: Program<'info, System>
}

impl<'info> InitChoice<'info> {

  pub fn init_choice(
    &mut self,
    bumps: &InitChoiceBumps
  ) -> Result<()> {

    Ok(())
  }
}
