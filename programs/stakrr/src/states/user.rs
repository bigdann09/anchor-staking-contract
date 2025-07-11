use anchor_lang::prelude::*;

use crate::states::Pool;

#[account]
#[derive(InitSpace)]
pub struct UserStakeInfo {
    pub owner: Pubkey,
    pub staked_amount: u64,
    pub reward_debt: u128,
    pub pending_rewards: u64,
    pub last_update_time: i64,
    pub bump: u8,
}

impl UserStakeInfo {
    pub fn claim_accrued_rewards(&mut self, pool: &Pool) {
        if self.staked_amount == 0 {
            self.reward_debt = pool.accrued_reward_per_share;
            return;
        }

        let current_accrued = pool.accrued_reward_per_share;
        let reward_per_share_earned = current_accrued.checked_sub(self.reward_debt).unwrap_or(0);

        let reward_earned_this_period = (self.staked_amount as u128)
            .checked_mul(reward_per_share_earned)
            .unwrap()
            .checked_div(Pool::SCALING_FACTOR)
            .unwrap_or(0); // scale down to actual token amount

        self.pending_rewards = self.pending_rewards
            .checked_add(reward_earned_this_period as u64)
            .unwrap();

        // update reward debt to current global accumulator
        self.reward_debt = current_accrued;
    }
}