use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Comic {
  pub creator: Pubkey,
  pub collection: Pubkey,
  #[max_len(100)]
  pub title: String,
  pub published: bool,
  pub bump: u8
}
