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

//! Messages for interacting with the [crate::contract].
use crate::{
    crypto::Sig,
    types::{ChannelId, FundingId, Params, State, Withdrawal},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Message to initialize the [crate::contract].
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InitMsg {}

/// Message to call functions on the [crate::contract].
///
/// Each message corresponds to one function.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Deposits funds into a channel for a specific [FundingId].
    ///
    /// Adds the newly deposited amount to already existing deposits.
    /// Funds that are deposited to an invalid `funding_id` will be lost.
    /// Over-funding a channel can result in lost funds as well.
    Deposit(FundingId),
    /// Disputes a channel in case of a dishonest participant.
    ///
    /// Can only be called with a non-finalized state that is signed by
    /// all participants.
    /// Once a dispute is started, anyone can dispute the channel again
    /// with a state that has a higher [State::version].
    /// A dispute automatically starts a timeout of [Params::dispute_duration]
    /// and can only be re-disputed while it did not run out.
    /// [ExecuteMsg::Conclude] can be called after the timeout ran out.
    Dispute(Params, State, Vec<Sig>),
    /// Collaboratively concludes a channel in one step.
    ///
    /// This function concludes a channel in the case that all participants
    /// want to close it.
    /// Can only be called with a [State::finalized] state that is signed by
    /// all participants.
    Conclude(Params, State, Vec<Sig>),
    /// Concluded a disputed channel.
    ///
    /// Can only be called after the timeout of the dispute ran out or if
    /// a [State::finalized] state is provided and signed by all participants.
    ConcludeDispute(Params),
    /// Withdraws funds from a concluded channel.
    ///
    /// Can be called by each participant after a channel was concluded to
    /// withdraw his outcome of the channel.
    /// This is the counterpart to [ExecuteMsg::Deposit].
    Withdraw(Withdrawal, Sig),
}

/// Message to query the state of the [crate::contract].
///
/// Each message corresponds to one query.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // Returns the on-chain deposit for a participant in a channel.
    Deposit(FundingId),
    // Returns the on-chain dispute for a channel.
    Dispute(ChannelId),
}
