use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked, mint_to, MintTo
}};

use crate::{error::StakingError, states::{Pool, UserStakeInfo}};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[b"pool"],
        bump=pool.bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init_if_needed,
        payer=signer,
        space=8 + UserStakeInfo::INIT_SPACE,
        seeds=[b"user", signer.key().as_ref(), pool.key().as_ref()],
        bump
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
        init_if_needed,
        payer=signer,
        associated_token::mint=lst_token_mint,
        associated_token::authority=signer
    )]
    pub user_lst_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

pub fn handle_stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let user_stake_info = &mut ctx.accounts.user_stake_info;
    let clock = Clock::get();

    require!(!pool.is_paused, StakingError::Paused);
    require!(amount > 0, StakingError::ZeroStakeAmount);

    // TODO: update global reward accumulator and user's pending rewards

    // TODO: claim accured reward

    // transfer staked tokens from user to vault
    msg!("transfer stake token amount {} to staking vault", amount);
    transfer_checked(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.user_staked_token_account.to_account_info(),
            to: ctx.accounts.staked_token_vault.to_account_info(),
            mint: ctx.accounts.staked_token_mint.to_account_info(),
            authority: ctx.accounts.signer.to_account_info()
        }),
        amount,
        ctx.accounts.staked_token_mint.decimals
    )?;

    // Mint LST to the user
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"pool",
        &[pool.bump]
    ]];

    msg!("minted {} LST tokens to user", amount);
    mint_to(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.lst_token_mint.to_account_info(),
            to: ctx.accounts.user_lst_token_account.to_account_info(),
            authority: pool.to_account_info()
        }, signer_seeds),
        amount
    )?;

    // update or initialize user stake info
    user_stake_info.owner = *ctx.accounts.signer.key;
    user_stake_info.staked_amount = user_stake_info.staked_amount.checked_add(amount).ok_or(StakingError::MathOverflow)?;
    user_stake_info.last_update_time = clock?.unix_timestamp;
    user_stake_info.bump = ctx.bumps.user_stake_info;

    // update total staked amount
    pool.total_staked_amount = pool.total_staked_amount.checked_add(amount).ok_or(StakingError::MathOverflow)?;

    Ok(())
}