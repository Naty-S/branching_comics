use anchor_lang::prelude::*;

use crate::{
  errors::ComicErrors,
  state::{User, Comic}
};


#[derive(Accounts)]
pub struct PublishComic<'info> {
  
  #[account(mut)]
  pub creator: Signer<'info>,

  #[account(
    mut,
    has_one = creator,
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

    // require!(self.comic.creator == self.user.key(), ComicErrors::NotComicCreator);
    require!(self.user.creator == true, ComicErrors::NotCreator);

    self.comic.published = true;

    Ok(())
  }
}
