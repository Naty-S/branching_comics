use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use mpl_core::{instructions::TransferV1CpiBuilder};

use crate::state::Chapter;


#[derive(Accounts)]
pub struct ChapterPurchase<'info> {

  #[account(mut)]
  pub buyer: Signer<'info>, // User
  
  #[account(mut)]
  pub seller: SystemAccount<'info>,
  
  // ==========
  // PDA's
  // ==========

  #[account(
    mut,
    has_one = mint, // how to represent in arch diagram?
    seeds = [
      b"chapter",
      chapter.mint.key().as_ref(),
      chapter.comic.key().as_ref()
    ],
    bump = chapter.bump
  )]
  pub chapter: Account<'info, Chapter>,
  
  // ==========
  // Metaplex core
  // ==========
  
  /// CHECK: This is the mint account (MPL Core NFT) of the chapter
  #[account(
    mut,
    // constraint = mint.key() == chapter.mint // @ ComicErrors::AssetMismatch,
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
    bump,
    // constraint = asset.owner == escrow.key() @ ComicError::AssetNotInEscrow,
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

impl<'info> ChapterPurchase<'info> {

  pub fn pay_seller(&mut self) -> Result<()> {

    let accounts = Transfer {
      from: self.buyer.to_account_info(),
      to: self.seller.to_account_info(),
    };

    let ctx = CpiContext::new(
      self.system_program.to_account_info(), 
      accounts
    );

    transfer(ctx, self.chapter.price)
  }

  pub fn send_chapter(&mut self, bumps: &ChapterPurchaseBumps) -> Result<()> {

    let vault_seeds: [&[&[u8]]; 1] = [&[
      b"chapter_vault",
      self.chapter.to_account_info().key.as_ref(),
      &[bumps.chapter_vault]
    ]];

    TransferV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
      .asset(&self.mint.to_account_info())
      .authority(Some(&self.chapter_vault.to_account_info()))
      .new_owner(&self.buyer.to_account_info())
      .payer(&self.buyer.to_account_info())
      .collection(Some(&self.collection_comic.to_account_info()))
      .system_program(Some(&self.system_program.to_account_info()))
      .invoke_signed(&vault_seeds)?;

    self.chapter.owner = self.buyer.key();
    
    Ok(())
  }
}
