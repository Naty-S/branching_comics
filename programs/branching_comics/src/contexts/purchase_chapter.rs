use anchor_lang::prelude::*;

use crate::Chapter;


#[derive(Accounts)]
pub struct PurchaseChapter<'info> {
  
  #[account(mut)]
  pub user: Signer<'info>,

  pub system_program: Program<'info, System>
}

impl<'info> PurchaseChapter<'info> {

  pub fn purchase_chapter(
    &mut self,
    bumps: &PurchaseChapterBumps
  ) -> Result<()> {

    Ok(())
  }
}
