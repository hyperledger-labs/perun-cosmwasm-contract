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

//! Core functionality for controlling the on-chain part of state channels.
use crate::{
    crypto::{OffIdentity, Sig},
    ensure,
    error::ContractError,
    msg::{ExecuteMsg, InitMsg, QueryMsg},
    storage::{DEPOSITS, DISPUTES},
    types::*,
};
use cosmwasm_std::{
    entry_point, BankMsg::Send, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Storage, Timestamp,
};
use std::{ops::Add, result::Result};

/// Handles all [InitMsg] messages.
///
/// Can be used to initialize the contract, which is only done once.
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> StdResult<Response> {
    Ok(Default::default())
}

/// Handles all [ExecuteMsg] messages.
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit { funding_id } => deposit(deps.storage, info, funding_id),
        ExecuteMsg::Dispute {
            params,
            state,
            sigs,
        } => dispute(deps.storage, env.block.time, &params, &state, &sigs),
        ExecuteMsg::Conclude {
            params,
            state,
            sigs,
        } => conclude(deps.storage, &params, &state, &sigs),
        ExecuteMsg::ConcludeDispute { params } => {
            conclude_dispute(deps.storage, env.block.time, &params)
        }
        ExecuteMsg::Withdraw { withdrawal, sig } => withdraw(deps.storage, &withdrawal, &sig),
    }
}

/// Handles all [QueryMsg] messages.
///
/// Can be used to query the contract state.
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Deposit { funding_id } => query_deposit(deps, funding_id),
        QueryMsg::Dispute { channel_id } => query_dispute(deps, channel_id),
    }
}

/// See [crate::msg::QueryMsg::Deposit].
fn query_deposit(deps: Deps, fid: FundingId) -> Result<Binary, ContractError> {
    match DEPOSITS.may_load(deps.storage, fid)? {
        Some(deposit) => {
            let bin = encode_obj(&deposit);
            ensure!(
                bin.is_some(),
                ContractError::InternalError("could not encode object".into())
            );
            Ok(bin.unwrap().into())
        }
        None => Err(ContractError::UnknownChannel {}),
    }
}

/// See [crate::msg::QueryMsg::Dispute].
fn query_dispute(deps: Deps, cid: ChannelId) -> Result<Binary, ContractError> {
    match DISPUTES.may_load(deps.storage, cid)? {
        Some(dispute) => {
            let bin = encode_obj(&dispute);
            ensure!(
                bin.is_some(),
                ContractError::InternalError("could not encode object".into())
            );
            Ok(bin.unwrap().into())
        }
        None => Err(ContractError::UnknownDispute {}),
    }
}

/// See [crate::msg::ExecuteMsg::Deposit].
fn deposit(
    storage: &mut dyn Storage,
    info: MessageInfo,
    funding_id: FundingId,
) -> Result<Response, ContractError> {
    DEPOSITS.update(storage, funding_id, |holding| -> Result<_, ContractError> {
        Ok(holding.unwrap_or_default().add(&info.funds.into()))
    })?;
    Ok(Default::default())
}

/// See [crate::msg::ExecuteMsg::Dispute].
fn dispute(
    storage: &mut dyn Storage,
    now: Timestamp,
    params: &Params,
    state: &State,
    sigs: &[Sig],
) -> Result<Response, ContractError> {
    ensure!(!state.finalized, ContractError::StateFinal {});
    state.verify_fully_signed(params, sigs)?;
    let channel_id = state.channel_id.clone();

    match DISPUTES.may_load(storage, channel_id.clone())? {
        None => {
            let timeout = now.plus_seconds(params.dispute_duration);
            let dispute = Dispute::Active {
                state: state.clone(),
                timeout,
            };
            DISPUTES.save(storage, channel_id, &dispute)?;
            Ok(Default::default())
        }
        Some(Dispute::Active {
            state: old_state,
            timeout,
        }) => {
            ensure!(
                state.version > old_state.version,
                ContractError::DisputeVersionTooLow {}
            );
            ensure!(now <= timeout, ContractError::DisputeTimedOut {});

            let dispute = Dispute::Active {
                state: state.clone(),
                timeout,
            };
            DISPUTES.save(storage, channel_id, &dispute)?;
            Ok(Default::default())
        }
        Some(Dispute::Concluded { .. }) => Err(ContractError::AlreadyConcluded {}),
    }
}

/// See [crate::msg::ExecuteMsg::Conclude].
fn conclude(
    storage: &mut dyn Storage,
    params: &Params,
    state: &State,
    sigs: &[Sig],
) -> Result<Response, ContractError> {
    ensure!(state.finalized, ContractError::StateNotFinal {});
    state.verify_fully_signed(params, sigs)?;
    let channel_id = &state.channel_id;

    match DISPUTES.may_load(storage, channel_id.clone())? {
        Some(Dispute::Concluded { .. }) => Err(ContractError::AlreadyConcluded {}),
        Some(Dispute::Active { .. }) => Err(ContractError::DisputeActive {}),
        None => {
            push_outcome(storage, channel_id, &params.participants, &state.balances)?;
            let reg = Dispute::Concluded {
                state: state.clone(),
            };
            DISPUTES.save(storage, channel_id.clone(), &reg)?; // TODO maybe use error into?
            Ok(Default::default())
        }
    }
}

/// See [crate::msg::ExecuteMsg::ConcludeDispute].
fn conclude_dispute(
    storage: &mut dyn Storage,
    now: Timestamp,
    params: &Params,
) -> Result<Response, ContractError> {
    let channel_id = params.channel_id()?;
    match DISPUTES.may_load(storage, channel_id.clone())? {
        None => Err(ContractError::UnknownDispute {}),
        Some(Dispute::Concluded { .. }) => Err(ContractError::AlreadyConcluded {}),
        Some(Dispute::Active { state, timeout }) => {
            // Check that the timeout has elapsed for non-final states.
            ensure!(
                state.finalized || now > timeout,
                ContractError::ConcludedTooEarly {}
            );
            // Write the outcome of the channel.
            push_outcome(storage, &channel_id, &params.participants, &state.balances)?;
            // End the dispute.
            DISPUTES.save(storage, channel_id, &Dispute::Concluded { state })?;
            Ok(Default::default())
        }
    }
}

/// See [crate::msg::ExecuteMsg::Withdraw].
fn withdraw(
    storage: &mut dyn Storage,
    withdrawal: &Withdrawal,
    withdrawal_sig: &Sig,
) -> Result<Response, ContractError> {
    withdrawal.verify(withdrawal_sig)?;
    // Load the dispute.
    match DISPUTES.may_load(storage, withdrawal.channel_id.clone())? {
        None => Err(ContractError::UnknownChannel {}),
        Some(Dispute::Active { .. }) => Err(ContractError::NotConcluded {}),
        Some(Dispute::Concluded { .. }) => {
            let funding_id = withdrawal.funding_id()?;
            // Load the deposit.
            let deposit = DEPOSITS.may_load(storage, funding_id.clone())?;
            ensure!(deposit.is_some(), ContractError::UnknownDeposit {});
            let deposit = deposit.unwrap();
            // Remove the deposit.
            DEPOSITS.remove(storage, funding_id);
            // Transfer the outcome to the user.
            let transfer = Send {
                to_address: withdrawal.receiver.clone().into_string(),
                amount: deposit.into(),
            };
            Ok(Response::new().add_message(transfer))
        }
    }
}

/// Pushes the outcome of a channel back into the `DEPOSITS` map.
///
/// Checks that the sum of outcome is smaller or equal to the sum
/// of deposits in the channel.
/// This ensures that the participants cannot withdraw more than they
/// initially deposited.
fn push_outcome(
    storage: &mut dyn Storage,
    channel_id: &ChannelId,
    parts: &[OffIdentity],
    outcome: &[NativeBalance],
) -> Result<Response, ContractError> {
    ensure!(
        parts.len() == outcome.len(),
        ContractError::InvalidOutcome {}
    );
    ensure!(!parts.is_empty(), ContractError::InvalidOutcome {});

    // Save all Funding IDs for later.
    let mut fids = Vec::<FundingId>::default();
    // Calculate the sums of the outcome and deposit.
    let mut sum_outcome = NativeBalance::default();
    let mut sum_deposit = NativeBalance::default();

    for (i, part) in parts.iter().enumerate() {
        let fid = calc_funding_id(channel_id, part)?;
        fids.push(fid.clone());
        let deposit = DEPOSITS.load(storage, fid).unwrap_or_default();

        sum_outcome = sum_outcome.add(&outcome[i]);
        sum_deposit = sum_deposit.add(&deposit);
    }
    // Ensure that the participants of a channel can never withdraw more
    // than their initially deposited.
    ensure!(
        sum_deposit.greater_or_equal(&sum_outcome),
        ContractError::InsufficientDeposits {}
    );
    // Over-funding a channel will result in lost funds.
    // Now we split up all funds according to the outcome.
    for (i, fid) in fids.iter().enumerate() {
        DEPOSITS.save(storage, fid.to_vec(), &outcome[i])?;
    }
    Ok(Default::default())
}
