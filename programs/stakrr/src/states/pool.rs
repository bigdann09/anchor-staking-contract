use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub authority: Pubkey,
    pub reward_token_mint: Pubkey,
    pub staked_token_mint: Pubkey,
    pub lst_token_mint: Pubkey,
    pub reward_token_vault: Pubkey,
    pub staked_token_vault: Pubkey,
    pub lst_token_vault: Pubkey,
    pub total_staked_amount: u64,
    pub reward_rate_per_second: u64,
    pub accured_reward_per_share: u64,
    pub reward_vault_bump: u8,
    pub staked_vault_bump: u8,
    pub lst_vault_bump: u8,
    pub is_paused: bool,
    pub bump: u8,
}

impl Pool {
    pub fn update_rewards() {
        
    }
}