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

//! Custom type definitions which model state channels.
use crate::{
    crypto::{hash, verify, Hash, OffIdentity, OnIdentity, Sig},
    ensure,
    error::ContractError,
};
use cosmwasm_std::{Coin, Timestamp, Api};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::Digest;
use std::ops::Add;

/// Uniquely identifies a channel.
///
/// Can be calculated with [Params::channel_id].
pub type ChannelId = Hash;
/// Uniquely identifies a participant in a channel.
///
/// Can be calculated with [calc_funding_id].
pub type FundingId = Hash;
/// Native balance of the protocol.
///
/// Holds balances for multiple assets.
#[derive(Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct NativeBalance(cw0::NativeBalance);

/// Used to encode a [NativeBalance].
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
struct EncodableBalance {
    pub coins: Vec<EncodableCoin>,
}

/// Used to encode a [cosmwasm_std::Coin].
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
struct EncodableCoin {
    pub denom: String,
    pub amount: [u8; 16],
}

/// Funding is used to encode a ChannelId with an OffIdentity
/// to allow for a reproducible way of calculating a FundingId.
#[derive(Serialize, Deserialize)]
pub struct Funding {
    pub channel: ChannelId,
    pub part: OffIdentity,
}

/// Random value that is used to make the [Params] of a channel unique.
pub type Nonce = Vec<u8>;
/// Timely duration in seconds.
pub type Seconds = u64;
/// State version counter.
pub type Version = u64;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
/// Fixed parameters of a channel.
///
/// Defines the [ChannelId] of a channel via [Params::channel_id].
pub struct Params {
    /// Nonce to make these Params unique. Should be picked randomly.
    pub nonce: Nonce,

    /// Participants of the channel.
    ///
    /// Contains the off-chain identities which are used to verify signatures
    /// for off-chain related crypto.
    pub participants: Vec<OffIdentity>,

    /// Challenge duration of the channel.
    ///
    /// Describes how long a dispute will be held open.
    pub dispute_duration: Seconds,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
/// Off-Chain state of a channel.
pub struct State {
    /// Unique channel ID.
    ///
    /// Is calculated from the channel's [Params] with [Params::channel_id].
    /// This locks all parameters in place and ensures that a participant
    /// that signed a state also signed the parameters of a channel.
    pub channel_id: ChannelId,

    /// Version of the state.
    ///
    /// Higher version states can override disputes with lower versions.
    /// An honest participant will never sign two state with the same version.
    pub version: Version,

    /// Balance of each participant in the channel.
    ///
    /// Must have the same length as [Params::participants].
    /// The balances of a final state describe the outcome
    /// of a channel and can then be withdrawn.
    pub balances: Vec<NativeBalance>,

    /// Whether or not this state is final.
    ///
    /// Final states define the last state of a channel.
    /// An honest participant will never sign another state after he signed a
    /// final state.
    pub finalized: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
/// Stores an on-chain dispute of a channel.
pub enum Dispute {
    /// Can be advanced with a higher version via `Dispute` as long as the
    /// timeout did not run out.
    Active {
        /// The state of the disputed channel.
        state: State,

        /// Timeout of the dispute.
        timeout: Timestamp,
    },
    /// Can only be withdrawn from since the timeout ran out.
    Concluded {
        /// The state of the disputed channel.
        state: State,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
/// Withdrawal authorization for on-chain funds.
///
/// This is signed by an off-chain participant too authorize
/// on-chain funds withdrawal to a specific on-chain account.
///
/// NOTE: The signature is not part of the struct.
pub struct Withdrawal {
    /// Channel from with to withdraw.
    pub channel_id: ChannelId,

    /// Off-chain participant to debit.
    pub part: OffIdentity,

    /// On-Chain Account to credited.
    pub receiver: OnIdentity,
}

impl Params {
    /// Calculates the channel id from this Params.
    pub fn channel_id(&self) -> Result<ChannelId, ContractError> {
        let h = hash(self, vec![])?;
        Ok(h.finalize().to_vec())
    }
}

impl From<Vec<Coin>> for NativeBalance {
    fn from(cs: Vec<Coin>) -> Self {
        let mut raw = cw0::NativeBalance(cs);
        raw.normalize();
        Self(raw)
    }
}

impl From<NativeBalance> for Vec<Coin> {
    fn from(b: NativeBalance) -> Self {
        b.0.into_vec()
    }
}

impl Add<&NativeBalance> for NativeBalance {
    type Output = Self;

    fn add(mut self, other: &Self) -> Self::Output {
        self.0 += other.0.clone();
        self
    }
}

impl Serialize for NativeBalance {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut bals: EncodableBalance = Default::default();
        for coin in self.0 .0.iter() {
            let amount = coin.amount.u128().to_be_bytes();
            bals.coins.push(EncodableCoin {
                denom: coin.denom.clone(),
                amount,
            });
        }
        bals.serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for NativeBalance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        let bals = EncodableBalance::deserialize(deserializer)?;
        let mut coins: Vec<Coin> = Default::default();
        for coin in bals.coins.iter() {
            let data: [u8; 16] = coin.amount;
            let amount = u128::from_be_bytes(data);
            coins.push(Coin::new(amount, coin.denom.clone()));
        }
        Ok(NativeBalance::from(coins))
    }
}

impl NativeBalance {
    /// Models `self >= b`.
    /// Defines a non-strict partial order in the mathematical sense since
    /// there exist `a` and `b` where `¬(a >= b) ^ ¬(b >= a)`.
    /// Only works with normalized inputs.
    pub fn greater_or_equal(&self, b: &NativeBalance) -> bool {
        b.0 .0.iter().map(|b| self.0.has(b)).all(|x| x)
    }
}

impl State {
    /// Verifies that `from` signed this State.
    pub fn verify(&self, sig: &Sig, from: &OffIdentity, api: &dyn Api) -> Result<(), ContractError> {
        verify(self, from, sig, api)
    }
    /// Verifies that all participants signed this State.
    pub fn verify_fully_signed(&self, params: &Params, sigs: &[Sig], api: &dyn Api) -> Result<(), ContractError> {
        // Check that the State and Params match.
        let channel_id = params.channel_id()?;
        ensure!(
            self.channel_id == channel_id,
            ContractError::WrongChannelId {}
        );
        // Channels without participants are invalid.
        ensure!(!sigs.is_empty(), ContractError::InvalidSignatureNum {});
        // Check the state signatures.
        ensure!(
            sigs.len() == params.participants.len(),
            ContractError::WrongSignatureNum {}
        );
        for (i, sig) in sigs.iter().enumerate() {
            self.verify(sig, &params.participants[i], api)?;
        }
        Ok(())
    }
}

impl Withdrawal {
    /// Verifies that `from` signed this Withdrawal.
    pub fn verify(&self, sig: &Sig, api: &dyn Api) -> Result<(), ContractError> {
        verify(self, &self.part, sig, api)
    }
    // Calculates the funding id from this Withdrawal.
    pub fn funding_id(&self) -> Result<FundingId, ContractError> {
        calc_funding_id(&self.channel_id, &self.part)
    }
}

/// Calculates the funding ID for a participant in a channel.
///
/// Returns the hash of the `ChannelId` concatenated with `OffIdentity`.
/// Must be consistent with the go-perun connector.
pub fn calc_funding_id(
    channel: &ChannelId,
    part: &OffIdentity,
) -> Result<FundingId, ContractError> {
    let digest = hash(
        &Funding {
            channel: channel.clone(),
            part: part.clone(),
        },
        vec![],
    )?;
    Ok(digest.finalize().to_vec())
}

/// Defines how objects are encoded in Perun CosmWASM.
///
/// Encoding can be swapped here easily, loot at
/// <https://serde.rs/#data-formats> for a list of formats.
/// Must be consistent with the go-perun connector.
pub fn encode_obj<T: Serialize>(obj: &T) -> Option<Vec<u8>> {
    bcs::to_bytes(obj).ok()
}

/// Defines how objects are decoded in Perun CosmWASM.
///
/// Placed here for easy access but could also be places in test/common/.
//#[cfg(test)]
pub fn decode_obj<'a, T>(raw: &'a [u8]) -> Option<T>
where
    T: Deserialize<'a>,
{
    bcs::from_bytes(raw).ok()
}
