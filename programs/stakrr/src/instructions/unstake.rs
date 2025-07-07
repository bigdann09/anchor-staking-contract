use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{burn, transfer_checked, Burn, Mint, TokenAccount, TransferChecked, TokenInterface}};

use crate::{error::StakingError, states::{Pool, UserStakeInfo}};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds=[b"pool"],
        bump=pool.bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        seeds=[b"user", signer.key().as_ref(), pool.key().as_ref()],
        bump=user_stake_info.bump
    )]
    pub user_stake_info: Account<'info, UserStakeInfo>,

    #[account(address = pool.staked_token_mint @ StakingError::InvalidStakingMint)]
    pub staked_token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        address = pool.staked_token_vault @ StakingError::InvalidStakedVault
    )]
    pub staked_token_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=staked_token_mint,
        associated_token::authority=signer
    )]
    pub user_staked_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(address = pool.lst_token_mint @ StakingError::InvalidLSTMint)]
    pub lst_token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint=lst_token_mint,
        associated_token::authority=signer
    )]
    pub user_lst_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handle_unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let user_stake_info = &mut ctx.accounts.user_stake_info;
    let current_timestamp = Clock::get()?.unix_timestamp;

    // update global reward accumulator and user's pending rewards
    pool.update_accrued_rewards_per_share(current_timestamp);
    user_stake_info.claim_accrued_rewards(pool);

    require!(user_stake_info.staked_amount >= amount, StakingError::InsufficientAmount);
    require!(ctx.accounts.user_lst_token_account.amount >= amount, StakingError::InsufficientLSTAmount);

    // burn LST from the user
    burn(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
    Burn {
            mint: ctx.accounts.lst_token_mint.to_account_info(),
            from: ctx.accounts.user_lst_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info()
        }),
        amount
    )?;

    // transfer staked tokens from pool back to user
    let stake_mint_pubkey = ctx.accounts.staked_token_mint.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"pool",
        stake_mint_pubkey.as_ref(),
        &[pool.bump]
    ]];

    transfer_checked(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.staked_token_vault.to_account_info(),
            to: ctx.accounts.user_staked_token_account.to_account_info(),
            mint: ctx.accounts.staked_token_mint.to_account_info(),
            authority: pool.to_account_info()
        }, signer_seeds),
        amount,
        ctx.accounts.staked_token_mint.decimals
    )?;

    // update user stake info
    user_stake_info.staked_amount = user_stake_info.staked_amount
        .checked_sub(amount)
        .ok_or(StakingError::MathOverflow)?;
    user_stake_info.reward_debt = pool.accrued_reward_per_share; // update debt to current global accumulator

    // update total staked amount in pool
    pool.total_staked_amount = pool.total_staked_amount
        .checked_sub(amount)
        .ok_or(StakingError::MathOverflow)?;

    Ok(())
}