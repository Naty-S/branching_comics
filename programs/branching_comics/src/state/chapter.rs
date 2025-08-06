use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Chapter {
  pub owner: Pubkey,
  pub comic: Pubkey,
  pub mint: Pubkey,
  pub next: Option<Pubkey>,
  pub start: bool,          // Determines if is the start chapter in a branch/path
  #[max_len(10)]
  pub choices: Vec<Pubkey>,
  pub price: u64,
  pub comic_bump: u8,
  pub bump: u8
}
