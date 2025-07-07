use anchor_lang::prelude::*;

use crate::error::StakingError;

#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds=[b"pool"],
        bump=pool.bump,
        constraint = pool.authority == signer.key() @ StakingError::Unauthorized,
        constraint = !pool.is_paused @ StakingError::AlreadyPaused,
    )]
    pub pool: Account<'info, crate::states::Pool>,
}

pub fn handle_pause(ctx: Context<Pause>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    // set paused state
    msg!("Pausing the staking pool...");
    pool.is_paused = true;

    msg!("Staking pool is now paused.");

    Ok(())
}