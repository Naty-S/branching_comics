use anchor_lang::prelude::*;

use crate::User;


#[derive(Accounts)]
#[instruction(is_creator: bool)]
pub struct InitUser<'info> {
  
  #[account(mut)]
  pub user: Signer<'info>,

  #[account(
    init,
    payer = user,
    seeds = [
      b"user",
      user.key().as_ref(),
      is_creator.to_string().as_bytes()
    ],
    space = 8 + User::INIT_SPACE,
    bump
  )]
  pub user_account: Account<'info, User>,

  pub system_program: Program<'info, System>
}

impl<'info> InitUser<'info> {

  pub fn init_user(&mut self, is_creator: bool, bumps: &InitUserBumps) -> Result<()> {

    self.user_account.set_inner(
      User {
        user: self.user.key(),
        creator: is_creator,
        bump: bumps.user_account
      }
    );

    Ok(())
  }
}
