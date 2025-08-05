use anchor_lang::prelude::*;

use crate::Chapter;


#[derive(Accounts)]
pub struct MakeChoice<'info> {
  
  #[account(mut)]
  pub user: Signer<'info>,

  pub system_program: Program<'info, System>
}

impl<'info> MakeChoice<'info> {

  pub fn make_chapter(
    &mut self,
    bumps: &MakeChoiceBumps
  ) -> Result<()> {

    Ok(())
  }
}
