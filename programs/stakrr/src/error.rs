use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("Invalid staking mint provided")]
    InvalidStakingMint,
    #[msg("Invalid staking vault account provided")]
    InvalidStakedVault,
    #[msg("Invalid LST mint provided")]
    InvalidLSTMint,
    #[msg("Program is paused")]
    Paused,
    #[msg("Amount must be greater than zero(0)")]
    ZeroStakeAmount,
    #[msg("")]
    MathOverflow
}