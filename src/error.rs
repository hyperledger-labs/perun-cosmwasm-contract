//  Copyright 2021 PolyCrypt GmbH
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

//! Custom error definitions which are returned by the contract functions.
use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Insufficient deposits")]
    InsufficientDeposits {},

    #[error("Unknown dispute")]
    UnknownDispute {},

    #[error("Unknown channel")]
    UnknownChannel {},

    #[error("Unknown deposit")]
    UnknownDeposit {},

    #[error("Dispute version too low")]
    DisputeVersionTooLow {},

    #[error("Dispute timed out")]
    DisputeTimedOut {},

    #[error("Already concluded")]
    AlreadyConcluded {},

    #[error("Concluded with different state")]
    ConcludedWithDifferentState {},

    #[error("Concluded too early")]
    ConcludedTooEarly {},

    #[error("Invalid signature: {0}")]
    InvalidSignature(String),

    #[error("Invalid identity")]
    InvalidIdentity {},

    #[error("Wrong signature")]
    WrongSignature {},

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
}

/// Automatic error wrapping for comprehensible code.
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) {
            return Err($e);
        }
    };
}
