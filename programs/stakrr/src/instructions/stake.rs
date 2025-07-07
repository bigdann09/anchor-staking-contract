use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{error::StakingError, states::Pool};

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

pub fn handle_stake(ctx: Context<Stake>) -> Result<()> {
    Ok(())
}