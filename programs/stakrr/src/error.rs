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
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Insufficient amount")]
    InsufficientAmount,
    #[msg("Insufficient LST amount")]
    InsufficientLSTAmount,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Insufficient funds in the user's account")]
    InsufficientFunds,
    #[msg("Pause operation already performed")]
    AlreadyPaused,
    #[msg("Unpause operation already performed")]
    AlreadyUnpaused,
    #[msg("Invalid reward token mint provided")]
    InvalidRewardTokenMint,
    #[msg("No rewards available to claim")]
    NoRewardsToClaim,
}