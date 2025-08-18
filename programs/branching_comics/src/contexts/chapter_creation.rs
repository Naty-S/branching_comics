use anchor_lang::prelude::*;
use mpl_core::{
  instructions::CreateV1CpiBuilder
  , types::{
      Attribute
    , Attributes
    , Plugin
    , PluginAuthorityPair
    , TransferDelegate
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
  pub parent: Option<Account<'info, Chapter>>,
  
  // The chapter itself will serve as the vault
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
    constraint = user_account.creator == true @ ComicErrors::NotCreator // only a creator can make chapters
  )]
  pub chapter: Account<'info, Chapter>,
  
  // ==========
  // Metaplex core
  // ==========
  
  /// CHECK: This is the mint account (MPL Core NFT) of the chapter
  // #[account(mut)]
  // pub mint: Signer<'info>,
  #[account(mut, signer)]
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
    if !is_start {
      let mut _parent = self.parent.clone().unwrap();
      // msg!("Linking chapter {} to parent {}", self.chapter.key(), _parent.key());
      _parent.next = Some(self.chapter.key());
      self.parent = Some(_parent);
      // msg!("Parent chapter: {}", &self.parent.as_ref().unwrap().next.unwrap());
    };

    Ok(())
  }

  pub fn mint_chapter(
    &mut self,
    name: String,
    uri: String,
    bumps: &ChapterCreationBumps
  ) -> Result<()> {

    require!(&self.chapter.comic == &self.comic.key(), ComicErrors::ChapterInvalidComic);
    require!(&self.chapter.mint == &self.mint.key(), ComicErrors::ChapterInvalidMint);

    let seeds: [&[&[u8]]; 1] = [&[
      b"authority",
      self.collection_comic.to_account_info().key.as_ref(),
      &[bumps.collection_comic_authority],
    ]];

    let attributes = PluginAuthorityPair {
      plugin: Plugin::Attributes(Attributes { attribute_list: vec![
        Attribute {
          key: "Name".to_string(), 
          value: name.to_string() 
        },
        Attribute {
          key: "Start".to_string(), 
          value: (&self.chapter.start).to_string()
        }
      ]}),
      authority: None
    };

    // Allows the chapter's tranfers by delegate only
    let transfer_delegate = PluginAuthorityPair {
      plugin: Plugin::TransferDelegate(TransferDelegate {}),
      authority: None
    };

    CreateV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
      .asset(&self.mint.to_account_info())
      .collection(Some(&self.collection_comic.to_account_info()))
      .authority(Some(&self.collection_comic_authority.to_account_info()))
      .payer(&self.user.to_account_info())
      .owner(Some(&self.user.to_account_info()))
      .update_authority(None)// Not changing the NFT (metadata) itself
      .system_program(&self.system_program.to_account_info())
      // .data_state(DataState::AccountState) -> this is the default
      .name(name)
      .uri(uri)
      .plugins(vec![
        attributes,
        transfer_delegate
      ])
      .invoke_signed(&seeds)?;

    msg!("Chapter minted successfully: {:?}", self.mint);
    Ok(())

  }
}
