use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::states::Pool;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer=signer,
        seeds=[b"pool"],
        space=8 + Pool::INIT_SPACE,
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub staked_token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer=signer,
        seeds=[b"staked_vault", staked_token_mint.key().as_ref()],
        token::mint=staked_token_mint,
        token::authority=pool,
        bump
    )]
    pub staked_token_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub reward_token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer=signer,
        seeds=[b"reward_vault", reward_token_mint.key().as_ref()],
        token::mint=reward_token_mint,
        token::authority=pool,
        bump
    )]
    pub reward_token_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer=signer,
        seeds=[b"lst_mint", staked_token_mint.key().as_ref()],
        mint::decimals=staked_token_mint.decimals,
        mint::authority=pool,
        bump
    )]
    pub lst_token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer=signer,
        seeds=[b"lst_vault", lst_token_mint.key().as_ref()],
        token::mint=lst_token_mint,
        token::authority=pool,
        bump
    )]
    pub lst_token_vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

pub fn handle_initialize_pool(ctx: Context<InitializePool>, rate_per_second: u64) -> Result<()> {
    ctx.accounts.pool.set_inner(Pool {
        authority: *ctx.accounts.signer.key,
        reward_token_mint: ctx.accounts.reward_token_mint.key(),
        staked_token_mint: ctx.accounts.staked_token_mint.key(),
        lst_token_mint: ctx.accounts.lst_token_mint.key(),
        reward_token_vault: ctx.accounts.reward_token_vault.key(),
        staked_token_vault: ctx.accounts.staked_token_vault.key(),
        lst_token_vault: ctx.accounts.lst_token_vault.key(),
        total_staked_amount: 0,
        reward_rate_per_second: rate_per_second,
        accured_reward_per_share: 0,
        reward_vault_bump: ctx.bumps.reward_token_vault,
        staked_vault_bump: ctx.bumps.staked_token_vault,
        lst_vault_bump: ctx.bumps.lst_token_vault,
        is_paused: false,
        bump: ctx.bumps.pool
    });
    Ok(())
}