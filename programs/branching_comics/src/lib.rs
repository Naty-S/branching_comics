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

    pub fn init_comic(ctx: Context<InitComic>, title: String) -> Result<()> {
        
        ctx.accounts.init_comic(title, &ctx.bumps)
    }

    pub fn publish_comic(ctx: Context<PublishComic>) -> Result<()> {
        
        ctx.accounts.publish_comic()
    }

    // ==========
    // Chapter
    // ==========

    pub fn init_chapter(ctx: Context<InitChapter>, is_start: bool) -> Result<()> {
        
        ctx.accounts.init_chapter(is_start, &ctx.bumps)
    }

    pub fn list_chapter(ctx: Context<ListChapter>, price: u64) -> Result<()> {
        
        ctx.accounts.list_chapter(price)
    }

    pub fn purchase_chapter(ctx: Context<PurchaseChapter>) -> Result<()> {
        
        Ok(())
    }

    // ==========
    // Choice
    // ==========
    
    pub fn init_choice(ctx: Context<InitChoice>, choice: String) -> Result<()> {
        
        ctx.accounts.init_choice(choice, &ctx.bumps)
    }

    pub fn make_choice(ctx: Context<MakeChoice>) -> Result<()> {
        
        Ok(())
    }
}


