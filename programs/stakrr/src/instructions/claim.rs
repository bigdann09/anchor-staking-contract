use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}};

use crate::{error::StakingError, states::{Pool, UserStakeInfo}};

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds=[b"pool"],
        has_one=reward_token_vault,
        bump=pool.bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        seeds=[b"user", signer.key().as_ref(), pool.key().as_ref()],
        bump=user_stake_info.bump
    )]
    pub user_stake_info: Account<'info, UserStakeInfo>,

    #[account(address = pool.reward_token_mint @ StakingError::InvalidRewardTokenMint)]
    pub reward_token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub reward_token_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = reward_token_mint,
        associated_token::authority = signer,
    )]
    pub user_reward_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handle_claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let signer = &ctx.accounts.signer;
    let user_stake_info = &mut ctx.accounts.user_stake_info;
    let current_timestamp = Clock::get()?.unix_timestamp;

    require!(!pool.is_paused, StakingError::Paused);


    // update global reward accumulator and user's pending rewards
    pool.update_accrued_rewards_per_share(current_timestamp);
    user_stake_info.claim_accrued_rewards(pool);

    let pending_rewards = user_stake_info.pending_rewards;
    require!(pending_rewards > 0, StakingError::NoRewardsToClaim);

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"pool",
        &[pool.bump],
    ]];

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.reward_token_vault.to_account_info(),
                to: ctx.accounts.user_reward_token_account.to_account_info(),
                mint: ctx.accounts.reward_token_mint.to_account_info(),
                authority: pool.to_account_info(),
            },
            signer_seeds,
        ),
        pending_rewards,
        ctx.accounts.reward_token_mint.decimals,
    )?;

    // reset pending rewards after claiming
    user_stake_info.pending_rewards = 0;
    user_stake_info.reward_debt = pool.accrued_reward_per_share; // update debt to current global accumulator

    msg!("Claimed {} rewards for user {}", pending_rewards, signer.key());
    Ok(())
}