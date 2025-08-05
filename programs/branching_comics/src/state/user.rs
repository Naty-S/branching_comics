use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
  pub bump: u8
}
