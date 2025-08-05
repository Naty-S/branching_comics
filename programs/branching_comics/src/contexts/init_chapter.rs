use anchor_lang::prelude::*;

use crate::Chapter;


#[derive(Accounts)]
pub struct InitChapter<'info> {
  
  #[account(mut)]
  pub user: Signer<'info>,

  pub system_program: Program<'info, System>
}

impl<'info> InitChapter<'info> {

  pub fn init_chapter(
    &mut self,
    bumps: &InitChapterBumps
  ) -> Result<()> {

    Ok(())
  }
}
