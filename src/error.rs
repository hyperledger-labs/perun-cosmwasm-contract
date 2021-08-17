use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Insufficient deposits")]
    InsufficientDeposits {},

    #[error("Unknown dispute")]
    UnknownDispute {},

    #[error("Unknown channel")]
    UnknownChannel {},

    #[error("Unknown deposit")]
    UnknownDeposit {},

    #[error("DisputeActive")]
    DisputeActive {},

    #[error("Dispute version too low")]
    DisputeVersionTooLow {},

    #[error("Dispute timed out")]
    DisputeTimedOut {},

    #[error("Already concluded")]
    AlreadyConcluded {},

    #[error("Concluded too early")]
    ConcludedTooEarly {},

    #[error("Invalid signature")]
    InvalidSignature {},

    #[error("Invalid identity")]
    InvalidIdentity {},

    #[error("Wrong public key recovered from signature")]
    WrongSignature {},

    #[error("Internal error, equivalent to a panic.")]
    InternalError(String),

    #[error("Wrong number of signatures)")]
    WrongSignatureNum {},

    #[error("Invalid number of signatures)")]
    InvalidSignatureNum {},

    #[error("Wrong channel id")]
    WrongChannelId {},

    #[error("Invalid outcome")]
    InvalidOutcome {},

    #[error("Outcome overflow")]
    OutcomeOverflow {},

    #[error("Demons mismatch")]
    DenomMismatch {},

    #[error("State not final")]
    StateNotFinal {},

    #[error("State final")]
    StateFinal {},

    #[error("Not concluded")]
    NotConcluded {},

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) {
            return Err($e);
        }
    };
}
