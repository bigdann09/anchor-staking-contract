use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;
pub mod error;

use instructions::*;

declare_id!("5GZcSx7NBYH6HEFiqGfSNJZVWz7qKvWaXR5Esk8UXxJU");

#[program]
pub mod stakrr {
    use super::*;

    pub fn initialize(ctx: Context<InitializePool>, rate_per_second: u64) -> Result<()> {
        handle_initialize_pool(ctx, rate_per_second)
    }
}