use anchor_lang::prelude::*;

use crate::error::StakingError;

#[derive(Accounts)]
pub struct Unpause<'info> {
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

pub fn handle_unpause(ctx: Context<Unpause>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    // set unpaused state
    msg!("Unpausing the staking pool...");
    pool.is_paused = false;

    msg!("Staking pool is now unpaused.");

    Ok(())
}