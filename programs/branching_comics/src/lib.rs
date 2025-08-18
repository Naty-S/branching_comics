#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

mod contexts;
mod state;
mod errors;

pub use contexts::*;
pub use state::*;

declare_id!("5YZMwaJFBUn3nwRocRYTm37qarZexjHd8pXmAbDrQjEg");

#[program]
pub mod branching_comics {
    use super::*;

    // ==========
    // User
    // ==========

    pub fn init_user(ctx: Context<InitUser>, is_creator: bool) -> Result<()> {
        
        ctx.accounts.init_user(is_creator, &ctx.bumps)
    }

    // ==========
    // Comic
    // ==========

    pub fn publish_new_comic(
        ctx: Context<ComicPublishing>,
        title: String,
        uri: String,
    ) -> Result<()> {
        
        ctx.accounts.init_comic(title, &ctx.bumps)?;
        ctx.accounts.create_comic(uri, &ctx.bumps)?;
        ctx.accounts.publish_comic()
    }

    // pub fn unpublish_comic(ctx: Context<UnpublishComic>) -> Result<()> {
        
    //     ctx.accounts.unpublish_comic()
    // }

    pub fn republish_comic(ctx: Context<ComicPublishing>) -> Result<()> {
        
        ctx.accounts.publish_comic()
    }

    // ==========
    // Chapter
    // ==========

    pub fn create_chapter(
        ctx: Context<ChapterCreation>,
        is_start: bool,
        name: String,
        uri: String
    ) -> Result<()> {
        
        ctx.accounts.init_chapter(is_start, &ctx.bumps)?;
        ctx.accounts.mint_chapter(name, uri, &ctx.bumps)
    }

    pub fn list_chapter(ctx: Context<ChapterListing>, price: u64) -> Result<()> {
        
        ctx.accounts.list_chapter(price)
    }

    pub fn purchase_chapter(ctx: Context<ChapterPurchase>) -> Result<()> {
        
        ctx.accounts.pay_seller()?;
        ctx.accounts.send_chapter(&ctx.bumps)
    }

    // ==========
    // Choice
    // ==========
    
    pub fn create_choice(ctx: Context<ChoiceCreation>, choice: String) -> Result<()> {
        
        ctx.accounts.init_choice(choice, &ctx.bumps)?;
        ctx.accounts.add_choice_to_chapter()
    }

    pub fn make_choice(ctx: Context<ChoiceSelection>, choice: String) -> Result<()> {
        
        ctx.accounts.make_choice(choice)
    }
}


