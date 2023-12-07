use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolNftError {
    #[error("unknown error")]
    Unknown,
    #[error("insufficient funds to send")]
    InsufficientFundsForTransaction,
}

impl From<SolNftError> for ProgramError {
    fn from(error: SolNftError) -> Self {
        match error {
            SolNftError::InsufficientFundsForTransaction => ProgramError::InsufficientFunds,
            _ => ProgramError::Custom(1),
        }
    }
}