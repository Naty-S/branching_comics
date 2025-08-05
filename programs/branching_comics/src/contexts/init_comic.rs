use anchor_lang::prelude::*;

use crate::{User, Comic};


#[derive(Accounts)]
#[instruction(title: String)]
pub struct InitComic<'info> {
  
  #[account(mut)]
  pub user: Signer<'info>, // Creator of the comic

  #[account(
    has_one = user,
    seeds = [
      b"user",
      user_account.user.key().as_ref(),
      user_account.is_creator.to_string().as_bytes()
    ],
    bump = user_account.bump
  )]
  pub user_account: Account<'info, User>,

  #[account(
    init,
    payer = creator,
    seeds = [
      b"comic",
      user.key().as_ref(),
      title.as_str().as_bytes()
    ],
    space = 8 + Comic::INIT_SPACE,
    bump,
    constraint = user_account.is_creator == true // only a creator can make comics
  )]
  pub comic: Account<'info, Comic>,

  pub system_program: Program<'info, System>
}

impl<'info> InitComic<'info> {

  pub fn init_comic(&mut self, title: String, bumps: &InitComicBumps) -> Result<()> {

    self.comic.set_inner(
      Comic {
        creator: self.user.key(),
        title,
        published: false,
        bump: bumps.comic,
      }
    );

    Ok(())
  }
}
