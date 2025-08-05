use anchor_lang::prelude::*;

use crate::Chapter;


#[derive(Accounts)]
pub struct ListChapter<'info> {
  
  #[account(mut)]
  pub user: Signer<'info>,

  pub system_program: Program<'info, System>
}

impl<'info> ListChapter<'info> {

  pub fn list_chapter(
    &mut self,
    bumps: &ListChapterBumps
  ) -> Result<()> {

    Ok(())
  }
}
