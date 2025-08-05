use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
  pub user: Pubkey,
  pub creator: bool,
  pub bump: u8
}
