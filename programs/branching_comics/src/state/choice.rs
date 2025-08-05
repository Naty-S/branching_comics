use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Choice {
  pub chapter: Pubkey,
  pub next_chapter: Pubkey,
  #[max_len(100)]
  pub choice: String,
  pub chapter_bump: u8,
  pub bump: u8
}
