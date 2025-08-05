use anchor_lang::prelude::*;

use crate::Chapter;


#[derive(Accounts)]
pub struct PublishComic<'info> {
  
  #[account(mut)]
  pub user: Signer<'info>,

  pub system_program: Program<'info, System>
}

impl<'info> PublishComic<'info> {

  pub fn publish_comic(
    &mut self,
    bumps: &PublishComicBumps
  ) -> Result<()> {

    Ok(())
  }
}
