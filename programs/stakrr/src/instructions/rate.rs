use anchor_lang::prelude::*;

use crate::error::StakingError;

#[derive(Accounts)]
pub struct UpdateRewardRate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"pool"],
        bump=pool.bump,
        constraint = pool.authority == signer.key() @ StakingError::Unauthorized,
    )]
    pub pool: Account<'info, crate::states::Pool>,
}

pub fn handle_update_reward_rate(ctx: Context<UpdateRewardRate>, new_rate: f64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let current_timestamp = Clock::get()?.unix_timestamp;

    require!(!pool.is_paused, StakingError::Paused);
    require!(new_rate != 0.0, StakingError::InvalidRateAmount);

    // update global reward accumulator before changing rate
    pool.update_accrued_rewards_per_share(current_timestamp);

    // set new reward rate
    pool.reward_rate_per_second = new_rate;

    Ok(())
}