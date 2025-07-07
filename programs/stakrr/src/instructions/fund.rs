use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::{error::StakingError, states::Pool};

#[derive(Accounts)]
pub struct FundRewardPool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds=[b"pool"],
        constraint=pool.authority == signer.key() @ StakingError::Unauthorized,
        has_one=reward_token_vault,
        bump=pool.bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(address = pool.reward_token_mint)]
    pub reward_token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub reward_token_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=reward_token_mint,
        associated_token::authority=signer
    )]
    pub admin_reward_token_account: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>
}

pub fn handle_fund_reward_pool(ctx: Context<FundRewardPool>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let current_timestamp = Clock::get()?.unix_timestamp;

    require!(!pool.is_paused, StakingError::Paused);
    require!(amount > 0, StakingError::ZeroStakeAmount);

    require!(ctx.accounts.admin_reward_token_account.amount >= amount, StakingError::InsufficientFunds);

    // update global reward accumulator before funding
    pool.update_accrued_rewards_per_share(current_timestamp);

    transfer_checked(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.admin_reward_token_account.to_account_info(),
            to: ctx.accounts.reward_token_vault.to_account_info(),
            mint: ctx.accounts.reward_token_mint.to_account_info(),
            authority: ctx.accounts.signer.to_account_info()
        }
    ), amount, ctx.accounts.reward_token_mint.decimals)?;

    Ok(())
}