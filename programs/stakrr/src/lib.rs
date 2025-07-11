use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;
pub mod error;

use instructions::*;

declare_id!("5GZcSx7NBYH6HEFiqGfSNJZVWz7qKvWaXR5Esk8UXxJU");

#[program]
pub mod stakrr {
    use super::*;

    pub fn initialize(ctx: Context<InitializePool>, rate_per_second: f64) -> Result<()> {
        handle_initialize_pool(ctx, rate_per_second)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        handle_stake(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        handle_unstake(ctx, amount)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        handle_claim_reward(ctx)
    }

    pub fn fund_reward_pool(ctx: Context<FundRewardPool>, amount: u64) -> Result<()> {
        handle_fund_reward_pool(ctx, amount)
    }

    pub fn update_reward_rate(ctx: Context<UpdateRewardRate>, new_rate: f64) -> Result<()> {
        handle_update_reward_rate(ctx, new_rate)
    }

    pub fn pause(ctx: Context<Pause>) -> Result<()> {
        handle_pause(ctx)
    }

    pub fn unpause(ctx: Context<Unpause>) -> Result<()> {
        handle_unpause(ctx)
    }
}