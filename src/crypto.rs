use crate::{
    ensure,
    error::ContractError,
    types::{encode_obj, Params, State},
};
use k256::ecdsa::{
    signature::{DigestVerifier, Signature as _},
    Signature, VerifyingKey,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Sig(pub Vec<u8>);
/// Off-Chain identity of a participant.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OffIdentity(pub Vec<u8>);

/// On-Chain identity of a participant.
pub type OnIdentity = cosmwasm_std::Addr;
pub type Hash = Vec<u8>;
pub type Hasher = Sha256;

/// Prepended to all messages before they are digested and signed.
/// Must be consistent with the go-perun connector.
pub const SIG_PREFIX: &[u8] = "GO-PERUN/COSMWASM".as_bytes();

/// Returns the digest of `Serialize` object.
///
/// Must be consistent with the go-perun connector.
pub fn hash<T: Serialize>(obj: &T) -> Result<Hasher, ContractError> {
    let encoded = encode_obj(obj);
    ensure!(
        encoded.is_some(),
        ContractError::InternalError("Object serialization failed.".into())
    );
    // Prepend the signature prefix.
    let data = [SIG_PREFIX, &encoded.unwrap()].concat();
    // Hash the data and assert the output length.
    Ok(Hasher::new().chain(&data))
}

/// Verify a signature on a `Serialize` object.
///
/// All validation is done by this method to allow easy swapping of
/// the signature algorithm.
/// Must be consistent with the go-perun connector.
pub fn verify<T: Serialize>(obj: &T, from: &OffIdentity, sig: &Sig) -> Result<(), ContractError> {
    // Decode the public key.
    let _pk = VerifyingKey::from_sec1_bytes(from.0.as_slice()); // TODO use as_slice everywhere
    ensure!(_pk.is_ok(), ContractError::InvalidIdentity {});
    let pk: VerifyingKey = _pk.unwrap();
    // Decode the signature.
    let s = Signature::from_bytes(sig.0.as_slice());
    ensure!(s.is_ok(), ContractError::InvalidSignature {});
    // Hash the data and verify the signature.
    let hash = hash(obj).unwrap();
    ensure!(
        pk.verify_digest(hash, &s.unwrap()).is_ok(),
        ContractError::WrongSignature {}
    );
    Ok(())
}

/// Verifies that `state_sigs` contains the signatures of all participants
/// on `state`.
pub fn verify_fully_signed(
    params: &Params,
    state: &State,
    state_sigs: &[Sig],
) -> Result<(), ContractError> {
    // Check that the State and Params match.
    let channel_id = params.channel_id()?;
    ensure!(
        state.channel_id == channel_id,
        ContractError::WrongChannelId {}
    );
    // Channels without participants are invalid.
    ensure!(
        !state_sigs.is_empty(),
        ContractError::InvalidSignatureNum {}
    );
    // Check the state signatures.
    ensure!(
        state_sigs.len() == params.participants.len(),
        ContractError::WrongSignatureNum {}
    );
    for (i, sig) in state_sigs.iter().enumerate() {
        state.verify(sig, &params.participants[i])?;
    }
    Ok(())
}
