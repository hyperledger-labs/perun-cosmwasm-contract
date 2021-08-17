use crate::{
    crypto::{hash, verify, Hash, OffIdentity, OnIdentity, Sig},
    error::ContractError,
};
use cosmwasm_std::{Coin, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::ops::Add;

pub const NONCE_LEN: usize = 32;

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
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct NativeBalance(cw0::NativeBalance);
/// Random value that is used to make the [Params] of a channel unique.
pub type Nonce = [u8; NONCE_LEN];
/// Time duration in seconds. Used in [Params::dispute_duration];
pub type Seconds = u64;
/// State version counter. Used in [State::version];
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
/// On-chain dispute of a channel.
pub enum Dispute {
    /// Can be advanced with a higher version state via `deposit`.
    Active {
        /// The state of the channel.
        state: State,

        /// Timeout of the dispute.
        timeout: Timestamp,
    },
    /// Concluded dispute that only allows a user to withdraw the outcome.
    Concluded {
        /// The state of the channel.
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
    pub fn channel_id(&self) -> Result<ChannelId, ContractError> {
        let h = hash(self)?;
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

impl NativeBalance {
    /// `a` has at least all coins and amounts that `b` has.
    /// Intuitively models `a >= b`.
    /// Only works with normalized inputs.
    pub fn greater_or_equal(&self, b: &NativeBalance) -> bool {
        self.0 .0.iter().map(|c| b.0.has(c)).any(|x| x)
    }
}

impl State {
    pub fn verify(&self, sig: &Sig, from: &OffIdentity) -> Result<(), ContractError> {
        verify(self, from, sig)
    }
}

impl Withdrawal {
    pub fn verify(&self, sig: &Sig) -> Result<(), ContractError> {
        verify(self, &self.part, sig)
    }

    pub fn funding_id(&self) -> Result<FundingId, ContractError> {
        calc_funding_id(&self.channel_id, &self.part)
    }
}

/// Calculates the funding ID of a participant in a channel.
/// Returns the hash of `channel` concatenated with `part`.
///
/// Must be consistent with the go-perun connector.
pub fn calc_funding_id(
    channel: &ChannelId,
    part: &OffIdentity,
) -> Result<FundingId, ContractError> {
    #[derive(Serialize)]
    struct Funding<'a> {
        channel: &'a ChannelId,
        part: &'a OffIdentity,
    }
    let digest = hash(&Funding { channel, part })?;
    Ok(digest.finalize().to_vec())
}

/// Defines how objects are encoded in Perun CosmWASM.
///
/// Encoding can be swapped here easily, loot at
/// <https://serde.rs/#data-formats> for a list of formats.
/// Must be consistent with the go-perun connector.
pub fn encode_obj<T: Serialize>(obj: &T) -> Option<Vec<u8>> {
    serde_json::to_vec(obj).ok()
    //bincode::serialize(obj).ok()
}

/*impl Balance {
    pub fn checked_add(self, other: &Self) -> Result<Self, ContractError> {
        ensure!(
            self.0.denom == other.0.denom,
            ContractError::DenomMismatch {}
        );
        let amount = self.0.amount.checked_add(other.0.amount)?;
        Ok(Balance::from(Coin {
            denom: self.0.denom,
            amount: amount,
        }))
    }

    pub fn from(v: Coin) -> Self {
        Self(v)
    }
}*/

/*
/// Represents a multi-asset balance.
impl MultiBalance {
    /// Checks that the two MultiBalances have the denoms.
    /// Returns their sum or an error.
    pub fn checked_add(self, other: &Self) -> Result<Self, ContractError> {
        if self == Self::default() {
            return Ok(other.clone());
        } else if *other == Self::default() {
            return Ok(self);
        }
        // Check that the Balances have the same length.
        // The denoms are checked in `Balance::checked_add`.
        ensure!(
            self.0.len() == other.0.len(),
            ContractError::DenomMismatch {}
        );
        let ret = self
            .0
            .iter()
            .enumerate()
            .map(|(i, s)| s.clone().checked_add(&other.0[i]))
            .collect::<Result<Vec<Balance>, ContractError>>()?;
        Ok(Self(ret))
    }

    pub fn from(v: Vec<Coin>) -> Self {
        Self(
            v.iter()
                .map(|c| Balance::from(c.clone()))
                .collect::<Vec<_>>(),
        )
    }

    pub fn as_vec(self) -> Vec<Coin> {
        self.0.iter().map(|c| c.clone().0).collect::<Vec<_>>()
    }
}
*/
