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
    pub reward_rate_per_second: f64,
    pub accrued_reward_per_share: u128,
    pub last_reward_update_timestamp: i64,
    pub reward_vault_bump: u8,
    pub staked_vault_bump: u8,
    pub lst_vault_bump: u8,
    pub is_paused: bool,
    pub bump: u8,
}

impl Pool {
    pub const SCALING_FACTOR: u128 = 1_000_000_000_000;

    pub fn update_accrued_rewards_per_share(&mut self, current_timestamp: i64) {
        if self.total_staked_amount == 0 {
            self.last_reward_update_timestamp = current_timestamp;
        }

        let elapsed_time = current_timestamp.checked_sub(self.last_reward_update_timestamp).unwrap_or(0);
        if elapsed_time == 0 {
            return;
        }

        // calculate rewards generated in this period
        let rewards_generated_this_period: u128 = (self.total_staked_amount as u128)
            .checked_mul(self.reward_rate_per_second as u128)
            .unwrap()
            .checked_mul(elapsed_time as u128)
            .unwrap();

        self.accrued_reward_per_share = self.accrued_reward_per_share
            .checked_add(
                rewards_generated_this_period
                    .checked_div(self.total_staked_amount as u128)
                    .unwrap_or(0)
            ).unwrap();

        self.last_reward_update_timestamp = current_timestamp;
    }
}