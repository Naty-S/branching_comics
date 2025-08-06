use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
      MasterEditionAccount
    , Metadata
    , MetadataAccount
  }
  , token_interface::{
      Mint
    , TokenInterface
  }
};

use crate::{
  User,
  Comic,
  Chapter
};


#[derive(Accounts)]
#[instruction(is_start: bool)]
pub struct InitChapter<'info> {
  
  // ==========
  // Related accounts
  // ==========

  #[account(mut)]
  pub user: Signer<'info>,

  #[account(
    has_one = user,
    seeds = [
      b"user",
      user_account.user.key().as_ref(),
      user_account.creator.to_string().as_bytes()
    ],
    bump = user_account.bump
  )]
  pub user_account: Account<'info, User>,

  #[account(
    seeds = [
      b"comic",
      comic.creator.key().as_ref(),
      comic.title.as_str().as_bytes()
    ],
    bump = comic.bump,
  )]
  pub comic: Account<'info, Comic>,

  #[account(
    mut,
    seeds = [
      b"chapter",
      parent.mint.key().as_ref(),
      parent.comic.key().as_ref()
    ],
    bump = parent.bump
  )]
  pub parent: Account<'info, Chapter>,
  
  // ==========
  // Chapter accounts
  // ==========
  
  #[account(
    init,
    payer = user,
    seeds = [
      b"chapter",
      mint.key().as_ref(),
      comic.key().as_ref()
    ],
    bump,
    space = 8 + Chapter::INIT_SPACE,
    constraint = user_account.creator == true // only a creator can make chapters
  )]
  pub chapter: Account<'info, Chapter>,
  
  pub mint: InterfaceAccount<'info, Mint>, // Mint of the chapter  
  pub collection_mint: InterfaceAccount<'info, Mint>,

  #[account(
    seeds = [ // follow metaplex convention
      b"metadata",
      metadata_program.key().as_ref(),
      mint.key().as_ref()
    ],
    seeds::program = metadata_program.key(),
    bump,
    constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
    constraint = metadata.collection.as_ref().unwrap().verified == true
  )]
  pub metadata: Account<'info, MetadataAccount>,

  #[account(
    seeds = [
      b"metadata",
      metadata_program.key().as_ref(),
      mint.key().as_ref(),
      b"edition"
    ],
    seeds::program = metadata_program.key(),
    bump,
  )]
  pub master_edition: Account<'info, MasterEditionAccount>,

  // ==========
  // Programs
  // ==========
  
  pub system_program: Program<'info, System>,
  pub token_program: Interface<'info, TokenInterface>,
  pub metadata_program: Program<'info, Metadata>
}

impl<'info> InitChapter<'info> {

  pub fn init_chapter(&mut self, is_start: bool, bumps: &InitChapterBumps) -> Result<()> {

    self.chapter.set_inner(
      Chapter {
        owner: self.user.key(),
        comic: self.comic.key(),
        mint: self.mint.key(),
        next: None,
        start: is_start,
        choices: Vec::new(),
        price: 0,
        comic_bump: self.comic.bump,
        bump: bumps.chapter,
      }
    );

    if !is_start { self.parent.next = Some(self.chapter.key()); };

    Ok(())
  }
}
