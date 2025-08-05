use anchor_lang::prelude::*;

use crate::User;


#[derive(Accounts)]
pub struct InitUser<'info> {
  
  #[account(mut)]
  pub user: Signer<'info>,

  pub system_program: Program<'info, System>
}

impl<'info> InitUser<'info> {

  pub fn init_user(
    &mut self,
    bumps: &InitUserBumps
  ) -> Result<()> {

    Ok(())
  }
}
