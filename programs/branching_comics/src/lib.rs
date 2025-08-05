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

    pub fn init_comic(ctx: Context<InitComic>) -> Result<()> {
        
        Ok(())
    }

    pub fn publish_comic(ctx: Context<PublishComic>) -> Result<()> {
        
        Ok(())
    }

    // ==========
    // Chapter
    // ==========

    pub fn init_chapter(ctx: Context<InitChapter>) -> Result<()> {
        
        Ok(())
    }

    pub fn list_chapter(ctx: Context<ListChapter>) -> Result<()> {
        
        Ok(())
    }

    pub fn purchase_chapter(ctx: Context<PurchaseChapter>) -> Result<()> {
        
        Ok(())
    }

    // ==========
    // Choice
    // ==========
    
    pub fn init_choice(ctx: Context<InitChoice>) -> Result<()> {
        
        Ok(())
    }

    pub fn make_choice(ctx: Context<MakeChoice>) -> Result<()> {
        
        Ok(())
    }
}


