use anchor_lang::prelude::*;
use mpl_core::{
  instructions::CreateCollectionV1CpiBuilder
};

use crate::{
  errors::ComicErrors,
  state::{User, Comic}
};


#[derive(Accounts)]
#[instruction(title: String)]
pub struct ComicPublishing<'info> {
  
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
    bump = user_account.bump,
    constraint = user_account.creator == true @ ComicErrors::NotCreator
  )]
  pub user_account: Account<'info, User>,

  #[account(
    init,
    payer = user,
    seeds = [
      b"comic",
      collection_comic.key().as_ref(),
      user.key().as_ref(),
      title.as_str().as_bytes()
    ],
    space = 8 + Comic::INIT_SPACE,
    bump,
    constraint = user_account.creator == true @ ComicErrors::NotCreator, // only a creator can make comics
  )]
  pub comic: Account<'info, Comic>,


  // ==========
  // Metaplex core
  // ==========
  
  /// CHECK: This is the Chaper's Collection and will be checked by the Metaplex Core program
  #[account(mut)]
  pub collection_comic: Signer<'info>,

  /// CHECK: This is the authority of the collection and it is unitialized
  #[account(
    seeds = [b"authority", collection_comic.key().as_ref()],
    bump,
  )]
  pub collection_comic_authority: UncheckedAccount<'info>,

  // ==========
  // Programs
  // ==========
  
  pub system_program: Program<'info, System>,

  /// CHECK: This is the ID of the Metaplex Core program
  #[account(address = mpl_core::ID)]
  pub mpl_core_program: UncheckedAccount<'info>
}

impl<'info> ComicPublishing<'info> {

  pub fn init_comic(&mut self, title: String, bumps: &ComicPublishingBumps) -> Result<()> {

    self.comic.set_inner(
      Comic {
        creator: self.user.key(),
        collection: self.collection_comic.key(),
        title,
        published: false,
        bump: bumps.comic,
      }
    );

    Ok(())
  }

  pub fn create_comic(
    &mut self,
    uri: String,
    bumps: &ComicPublishingBumps
  ) -> Result<()> {

    require!(&self.comic.creator == &self.user.key(), ComicErrors::NotComicCreator);

    let seeds: [&[&[u8]]; 1] = [&[
      b"authority",
      self.collection_comic.to_account_info().key.as_ref(),
      &[bumps.collection_comic_authority],
    ]];

    CreateCollectionV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
      .collection(&self.collection_comic.to_account_info())
      .update_authority(Some(&self.collection_comic_authority.to_account_info()))
      .payer(&self.user.to_account_info())
      .system_program(&self.system_program.to_account_info())
      .name(self.comic.title.clone())
      .uri(uri)
      .invoke_signed(&seeds)?;

    Ok(())
  }

  pub fn publish_comic(&mut self) -> Result<()> {

    require!(&self.comic.creator == &self.user.key(), ComicErrors::NotComicCreator);

    self.comic.published = true;

    Ok(())
  }
}
