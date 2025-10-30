use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum RatioFunError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Invalid account data")]
    InvalidAccountData,
    #[error("Math overflow")]
    MathOverflow,
    #[error("Poll not active")]
    PollNotActive,
    #[error("Poll not resolvable yet")]
    PollNotResolvable,
    #[error("Unauthorized")]
    Unauthorized,
}

impl From<RatioFunError> for ProgramError {
    fn from(e: RatioFunError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

