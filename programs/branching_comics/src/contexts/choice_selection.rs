use anchor_lang::prelude::*;

use crate::{
  errors::ComicErrors,
  state::{Chapter, Choice}
};


#[derive(Accounts)]
pub struct ChoiceSelection<'info> {
  
  #[account(mut)]
  pub user: Signer<'info>,

  #[account(
    mut,
    seeds = [
      b"chapter",
      chapter.mint.key().as_ref(),
      chapter.comic.key().as_ref()
    ],
    bump = chapter.bump,
  )]
  pub chapter: Account<'info, Chapter>,

  #[account(
    has_one = chapter,
    seeds = [
      b"choice",
      choice.chapter.key().as_ref(),
      choice.choice.as_str().as_bytes()
    ],
    bump = choice.bump
  )]
  pub choice: Account<'info, Choice>,

  pub system_program: Program<'info, System>
}

impl<'info> ChoiceSelection<'info> {

  pub fn make_choice(&mut self, choice: String) -> Result<()> {

    require!(self.chapter.owner == self.user.key(), ComicErrors::NotChapterOwner);
    require!(self.chapter.choices.len() > 0, ComicErrors::NoChoicesChapter);
    require!(self.chapter.next == None, ComicErrors::ChoiceSelected);
    require!(self.choice.choice == choice, ComicErrors::NoSelectedChoice);
    require!(self.chapter.choices.contains(&self.choice.key()), ComicErrors::NoSelectedChoice);

    self.chapter.next = Some(self.choice.next_chapter);

    Ok(())
  }
}
