use anchor_lang::prelude::*;
use mpl_core::{
    instructions::CreateV1CpiBuilder
  , types::{
      Attribute
    , Attributes
    // , DataState
    , PluginAuthorityPair
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
pub struct ChapterCreation<'info> {

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
    constraint = comic.published == true @ ComicErrors::NoPublishedComic // Only published comics can have chapters
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
  
  #[account(
    init,
    payer = user,
    has_one = mint,
    has_one = comic,
    seeds = [
      b"chapter",
      mint.key().as_ref(),
      comic.key().as_ref()
    ],
    bump,
    space = 8 + Chapter::INIT_SPACE,
    constraint = user_account.creator == true @ ComicErrors::NotCreator // only a creator can make chapters
  )]
  pub chapter: Account<'info, Chapter>,
  
  // ==========
  // Metaplex core
  // ==========
  
  /// CHECK: This is the mint account of the chapter
  #[account(mut)]
  pub mint: Signer<'info>,

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
  // Programs
  // ==========
  
  pub system_program: Program<'info, System>,

  /// CHECK: This is the ID of the Metaplex Core program
  #[account(address = mpl_core::ID)]
  pub mpl_core_program: UncheckedAccount<'info>
}

impl<'info> ChapterCreation<'info> {

  pub fn init_chapter(&mut self, is_start: bool, bumps: &ChapterCreationBumps) -> Result<()> {

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

    // Link this chapter to parent only if isn't the start of a branch
    if !is_start { self.parent.next = Some(self.chapter.key()); };

    Ok(())
  }

  pub fn mint_chapter(
    &mut self,
    name: String,
    uri: String,
    bumps: &ChapterCreationBumps
  ) -> Result<()> {

    let seeds: [&[&[u8]]; 1] = [&[
      b"authority",
      self.collection_comic.to_account_info().key.as_ref(),
      &[bumps.collection_comic_authority],
    ]];

    let attribute_list = vec![
      Attribute {
        key: "Name".to_string(), 
        value: name.to_string() 
      },
      Attribute {
        key: "Start".to_string(), 
        value: (&self.chapter.start).to_string()
      }
    ];

    CreateV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
      .asset(&self.mint.to_account_info())
      .collection(Some(&self.collection_comic.to_account_info()))
      .authority(Some(&self.collection_comic_authority.to_account_info()))
      .payer(&self.user.to_account_info())
      .owner(Some(&self.user.to_account_info()))
      .update_authority(None)// Not changing the NFT itself
      .system_program(&self.system_program.to_account_info())
      // .data_state(DataState::AccountState) -> this is the default
      .name(name)
      .uri(uri)
      .plugins(vec![PluginAuthorityPair {
        plugin: mpl_core::types::Plugin::Attributes(Attributes { attribute_list }),
        authority: None
      }])
      .invoke_signed(&seeds)?;

    Ok(())

  }
}
