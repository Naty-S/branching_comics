use anchor_lang::prelude::*;
use mpl_core::{
  instructions::{
    ApprovePluginAuthorityV1CpiBuilder, TransferV1CpiBuilder
  },
  types::{
    PluginAuthority, PluginType::TransferDelegate
  },
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
pub struct ChapterListing<'info> {
  
  #[account(mut)]
  pub user: Signer<'info>,

  // ==========
  // PDA's
  // ==========

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
      comic.collection.key().as_ref(),
      comic.creator.key().as_ref(),
      comic.title.as_str().as_bytes()
    ],
    bump = comic.bump,
    constraint = comic.published == true @ ComicErrors::NoPublishedComic // Only published comics can have chapters listed
  )]
  pub comic: Account<'info, Comic>,

  #[account(
    mut,
    has_one = mint,
    has_one = comic,
    seeds = [
      b"chapter",
      chapter.mint.key().as_ref(),
      chapter.comic.key().as_ref()
    ],
    bump = chapter.bump,
    constraint = chapter.owner == user.key() @ ComicErrors::NotChapterOwner,
  )]
  pub chapter: Account<'info, Chapter>,
  
  // ==========
  // Metaplex core
  // ==========
  
  /// CHECK: This is the mint account (MPL Core NFT) of the chapter
  #[account(
    mut,
    // constraint = *mint.owner == user.key() @ ComicErrors::NotMintOwner,
  )]
  pub mint: UncheckedAccount<'info>,

  /// CHECK: This is the Chaper's Collection and will be checked by the Metaplex Core program
  #[account(mut)]
  pub collection_comic: UncheckedAccount<'info>,

  /// CHECK: This is the authority of the collection and it is unitialized
  #[account(
    seeds = [b"authority", collection_comic.key().as_ref()],
    bump,
  )]
  pub collection_comic_authority: UncheckedAccount<'info>,

  // ==========
  // Escrow accounts
  // ==========

  /// CHECK:
  #[account(
    mut,
    seeds = [b"chapter_vault", chapter.key().as_ref()],
    bump
  )]
  pub chapter_vault: UncheckedAccount<'info>,

  
  // ==========
  // Programs
  // ==========
  
  pub system_program: Program<'info, System>,

  /// CHECK: This is the ID of the Metaplex Core program
  #[account(address = mpl_core::ID)]
  pub mpl_core_program: UncheckedAccount<'info>
}

impl<'info> ChapterListing<'info> {

  pub fn list_chapter(&mut self, price: u64) -> Result<()> {

    require!(self.chapter.owner == self.user.key(), ComicErrors::NotChapterOwner);
    require!(price > 0 , ComicErrors::InvalidChapterPrice);

    // Set chapter's price
    self.chapter.price = price;

    // Transfer chapter to vault
    // msg!("Transferring chapter to vault...");

    ApprovePluginAuthorityV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
      .plugin_type(TransferDelegate)
      .asset(&self.mint.to_account_info())
      .authority(Some(&self.user.to_account_info()))
      .new_authority(PluginAuthority::Address { address: self.chapter_vault.key() })
      .payer(&self.user.to_account_info())
      .collection(Some(&self.collection_comic.to_account_info()))
      .system_program(&self.system_program.to_account_info())
      .invoke()?;

    TransferV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
      .asset(&self.mint.to_account_info())
      .new_owner(&self.chapter_vault.to_account_info())
      .payer(&self.user.to_account_info())
      .collection(Some(&self.collection_comic.to_account_info()))
      .authority(Some(&self.user.to_account_info()))
      .system_program(Some(&self.system_program.to_account_info()))
      .invoke()?;
    
    // msg!("{:?}", self.mint);

    Ok(())
  }
}
