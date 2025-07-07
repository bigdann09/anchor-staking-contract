use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("Invalid staking mint provided")]
    InvalidStakingMint,
    #[msg("Invalid staking vault account provided")]
    InvalidStakedVault,
    #[msg("")]
    InvalidLSTMint
}