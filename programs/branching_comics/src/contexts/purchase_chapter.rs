use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken
  , token_interface::{
      CloseAccount
    , Mint
    , TokenAccount
    , TokenInterface
    , Transfer
    , TransferChecked
    , close_account
    , transfer
    , transfer_checked
  }
};

use crate::{state::Chapter};


#[derive(Accounts)]
pub struct PurchaseChapter<'info> {
  
  // ==========
  // Related accounts
  // ==========

  #[account(mut)]
  pub buyer: Signer<'info>, // User
  
  #[account(mut)]
  pub seller: SystemAccount<'info>,

  // ==========
  // Chapter accounts
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
  
  pub mint: InterfaceAccount<'info, Mint>, // Mint of the chapter

  /// CHECK: This is the Chaper's Collection and will be checked by the Metaplex Core program
  #[account(mut)]
  pub chapter_collection: UncheckedAccount<'info>,

  /// CHECK: This is the authority of the collection and it is unitialized
  #[account(
    seeds = [b"authority", chapter_collection.key().as_ref()],
    bump,
  )]
  pub chapter_collection_authority: UncheckedAccount<'info>,

  // ==========
  // Escrow accounts
  // ==========

  #[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = mint,
    associated_token::authority = buyer,
    associated_token::token_program = token_program
  )]
  pub buyer_chapter_ata: InterfaceAccount<'info, TokenAccount>,

  #[account(
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

  /// CHECK: This is the ID of the Metaplex Core program
  #[account(address = mpl_core::ID)]
  pub mpl_core_program: UncheckedAccount<'info>
}

impl<'info> PurchaseChapter<'info> {

  pub fn pay_seller(&mut self,) -> Result<()> {

    let accounts = Transfer {
      from: self.buyer.to_account_info(),
      to: self.seller.to_account_info(),
      authority: self.buyer.to_account_info()
    };

    let ctx = CpiContext::new(
      self.token_program.to_account_info(), 
      accounts
    );

    transfer(ctx, self.chapter.price)
  }

  pub fn send_chapter(&mut self,) -> Result<()> {
    
    let accounts = TransferChecked {
      from: self.chapter_vault.to_account_info(),
      to: self.buyer_chapter_ata.to_account_info(),
      mint: self.mint.to_account_info(),
      authority: self.chapter.to_account_info()
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

    transfer_checked(ctx, 1, self.mint.decimals)
  }

  pub fn close_vault(&mut self) -> Result<()> {

    let accounts = CloseAccount {
      account: self.chapter_vault.to_account_info(),
      destination: self.seller.to_account_info(),
      authority: self.chapter.to_account_info()
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

    close_account(ctx)
  }
}
