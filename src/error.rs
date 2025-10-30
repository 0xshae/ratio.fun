use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum CursedTakesError {
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

impl From<CursedTakesError> for ProgramError {
    fn from(e: CursedTakesError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

