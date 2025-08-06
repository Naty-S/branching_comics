use anchor_lang::prelude::*;

use crate::{
  errors::ComicErrors,
  state::{User, Comic}
};


#[derive(Accounts)]
pub struct PublishComic<'info> {
  
  #[account(mut)]
  pub user: Signer<'info>,

  #[account(
    has_one = user,
    seeds = [
      b"user",
      user_account.user.key().as_ref(),
      user_account.creator.to_string().as_bytes()
    ],
    bump = user_account.bump,
    constraint = user_account.creator == true
  )]
  pub user_account: Account<'info, User>,

  #[account(
    mut,
    seeds = [
      b"comic",
      comic.creator.key().as_ref(),
      comic.title.as_str().as_bytes()
    ],
    bump = comic.bump,
  )]
  pub comic: Account<'info, Comic>,

  pub system_program: Program<'info, System>
}

impl<'info> PublishComic<'info> {

  pub fn publish_comic(&mut self) -> Result<()> {

    require!(self.comic.creator == self.user.key(), ComicErrors::NotComicCreator);
    // require!(self.user_account.creator == true, ComicErrors::NotCreator);

    self.comic.published = true;

    Ok(())
  }
}
