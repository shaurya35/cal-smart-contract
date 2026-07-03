pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3XG3SjJFeobck2JCr3zgt7RiHJ2dGtqSsFqCxN5f4N4F");

#[program]
pub mod calculator {
    use super::*;

    pub fn init(){

    }

    pub fn double(){

    }

    pub fn add(num: u32){

    }
}

#[account]
struct DataShape {
    pub num: u32 
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 4)]
    pub account: Account<'info, DataShape>
    pub system_program: Program<'info, System>
    #[account(mut)]
    signer: Signer<'info>
}
