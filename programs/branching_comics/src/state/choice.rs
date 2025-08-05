use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Choice {
  pub bump: u8
}
