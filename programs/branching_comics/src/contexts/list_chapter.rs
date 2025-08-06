use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken
  , metadata::{
      MasterEditionAccount
    , Metadata
    , MetadataAccount
  }
  , token_interface::{
      Mint
    , MintTo
    , TokenAccount
    , TokenInterface
    , mint_to
  }
};

use crate::{
  errors::ComicErrors,
  state::{
      User
    , Comic
    , Chapter
  }
};

#[derive(Accounts)]
pub struct ListChapter<'info> {
  
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
    constraint = comic.published == true // Only published comics can have chapters listed
  )]
  pub comic: Account<'info, Comic>,

  // ==========
  // Chapter accounts
  // ==========
  
  #[account(
    mut,
    has_one = mint,
    has_one = comic,
    seeds = [
      b"chapter",
      chapter.mint.key().as_ref(),
      chapter.comic.key().as_ref()
    ],
    bump = chapter.bump
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
  // Escrow accounts
  // ==========

  #[account(
    init,
    payer = user,
    associated_token::mint = mint,
    associated_token::authority = chapter,
    associated_token::token_program = token_program
  )]
  pub chapter_vault: InterfaceAccount<'info, TokenAccount>,
  
  // ==========
  // Programs
  // ==========
  
  pub system_program: Program<'info, System>,
  pub token_program: Interface<'info, TokenInterface>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub metadata_program: Program<'info, Metadata>
}

impl<'info> ListChapter<'info> {

  pub fn list_chapter(&mut self, price: u64) -> Result<()> {

    require!(self.chapter.owner == self.user.key(), ComicErrors::NotChapterOwner);
    // require!(self.comic.published == true, ComicErrors::NoPublishedComic);

    // Set chapter's price
    self.chapter.price = price;

    // Mint chapter to vault (transfer)

    let accounts = MintTo {
      mint: self.mint.to_account_info(),
      to: self.chapter_vault.to_account_info(),
      authority: self.chapter.to_account_info(), // or user? the chapter is the escrow, right?
    };

    let seeds: [&[&[u8]]; 1] = [&[
      b"chapter",
      self.chapter.mint.as_ref(),
      self.chapter.comic.as_ref(),
      &[self.chapter.bump]
    ]];

    let ctx = CpiContext::new_with_signer(
      self.token_program.to_account_info(),
      accounts,
      &seeds
    );
    
    mint_to(ctx, 1)
  }
}
