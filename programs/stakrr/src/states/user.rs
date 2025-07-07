use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserStakeInfo {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub staked_amount: u64,
    pub reward_debt: u128,
    pub pending_rewards: u64,
    pub last_update_time: i64,
    pub bump: u8,
}