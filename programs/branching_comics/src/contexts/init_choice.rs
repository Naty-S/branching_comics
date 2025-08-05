use anchor_lang::prelude::*;

use crate::{
  User,
  Chapter,
  Choice
};


#[derive(Accounts)]
#[instruction(choice: String)]
pub struct InitChoice<'info> {
  
  #[account(mut)]
  pub user: Signer<'info>,

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
    seeds = [
      b"chapter",
      next_chapter.mint.key().as_ref(),
      next_chapter.comic.key().as_ref()
    ],
    bump = next_chapter.bump,
  )]
  pub next_chapter: Account<'info, Chapter>,

  #[account(
    init,
    payer = user,
    seeds = [
      b"choice",
      chapter.key().as_ref(),
      choice.as_str().as_bytes()
    ],
    bump,
    space = 8 + Choice::INIT_SPACE,
    constraint = user_account.is_creator == true // only a creator can create choices
  )]
  pub choice: Account<'info, Choice>,

  pub system_program: Program<'info, System>
}

impl<'info> InitChoice<'info> {

  pub fn init_choice(&mut self, choice: String, bumps: &InitChoiceBumps) -> Result<()> {

    self.choice.set_inner(
      Choice {
        chapter: self.chapter.key(),
        next_chapter: self.next_chapter.key(),
        choice,
        chapter_bump: bumps.chapter,
        bump: bumps.choice,
      }
    );

    self.chapter.choices.push(self.choice.key());
    
    Ok(())
  }
}
