use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken
  , token_interface::{
      Mint
    , TokenAccount
    , TokenInterface
    , TransferChecked
    , transfer_checked
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
  
  // The chapter (MPL Core NFT) to be listed
  pub mint: InterfaceAccount<'info, Mint>,

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

  #[account(
    init,
    payer = user,
    associated_token::mint = mint,
    associated_token::authority = chapter,
    associated_token::token_program = token_program
  )]
  pub chapter_vault: InterfaceAccount<'info, TokenAccount>,
  
  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = user,
    associated_token::token_program = token_program
  )]
  pub chapter_user_ata: InterfaceAccount<'info, TokenAccount>,
  
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

impl<'info> ListChapter<'info> {

  pub fn list_chapter(&mut self, price: u64) -> Result<()> {

    require!(price > 0 , ComicErrors::InvalidChapterPrice);

    // Set chapter's price
    self.chapter.price = price;

    // Transfer chapter to vault

    // if user has ATA -> transfer to vault
    // else -> mint chapter to vault

    let cpi_program = self.token_program.to_account_info();

    let cpi_accounts = TransferChecked {
        mint: self.mint.to_account_info()
      , from: self.chapter_user_ata.to_account_info()
      , to: self.chapter_vault.to_account_info()
      , authority: self.user.to_account_info()
    };

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    transfer_checked(cpi_ctx, self.chapter_user_ata.amount, self.mint.decimals) // amount should be 1
  }
}
