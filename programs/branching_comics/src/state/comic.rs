use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Comic {
  pub bump: u8
}
