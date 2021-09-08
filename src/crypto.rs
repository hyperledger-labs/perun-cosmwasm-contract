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

//! Cryptographic helpers for hashing and signature verification.
use crate::{ensure, error::ContractError, types::encode_obj};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use cosmwasm_std::Api;

/// Cryptographic signature.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Sig(pub Vec<u8>);
/// Off-Chain identity of a participant.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OffIdentity(pub Vec<u8>);

/// On-Chain identity of a participant.
pub type OnIdentity = cosmwasm_std::Addr;
/// Cryptographic hash.
pub type Hash = Vec<u8>;
/// Cryptographic hash function.
pub type Hasher = Sha256;

/// Prepended to all messages before they are digested and signed.
/// Must be consistent with the go-perun connector.
pub const SIG_PREFIX: &[u8] = "GO-PERUN/COSMWASM".as_bytes();

/// Returns the digest of `Serialize` object.
///
/// Must be consistent with the go-perun connector.
pub fn hash<T: Serialize>(obj: &T, prefix: Vec<u8>) -> Result<Hasher, ContractError> {
    let encoded = encode_obj(obj);
    ensure!(
        encoded.is_some(),
        ContractError::InternalError("Object serialization failed.".into())
    );
    // Prepend the signature prefix.
    let data = [prefix, encoded.unwrap()].concat();
    // Hash the data and assert the output length.
    Ok(Hasher::new().chain(&data))
}

/// Verify a signature on a `Serialize` object.
///
/// All validation is done by this method to allow easy swapping of
/// the signature algorithm.
/// Must be consistent with the go-perun connector.
pub fn verify<T: Serialize>(obj: &T, from: &OffIdentity, sig: &Sig, api: &dyn Api,) -> Result<(), ContractError> {
    let hasher = hash(obj, SIG_PREFIX.into())?;
    let hash = hasher.finalize();
    let ok = api.secp256k1_verify(&hash[..], sig.0.as_slice(), from.0.as_slice());
    ensure!(ok.is_ok(), ContractError::InvalidSignature {});
    ensure!(ok.unwrap(), ContractError::WrongSignature {});
    Ok(())
}
