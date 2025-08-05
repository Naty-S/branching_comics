use anchor_lang::prelude::*;

use crate::Comic;


#[derive(Accounts)]
pub struct InitComic<'info> {
  
  #[account(mut)]
  pub user: Signer<'info>,

  pub system_program: Program<'info, System>
}

impl<'info> InitComic<'info> {

  pub fn init_comic(
    &mut self,
    bumps: &InitComicBumps
  ) -> Result<()> {

    Ok(())
  }
}
