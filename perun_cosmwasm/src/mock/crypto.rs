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

use crate::{
    crypto::{hash, Sig, SIG_PREFIX},
    types::State,
};
use k256::ecdsa::{
    signature::{DigestSigner, Signature as _},
    SigningKey,
};
use serde::Serialize;

pub fn sign<T: Serialize>(obj: &T, sk: &SigningKey) -> Sig {
    let h = hash(obj, SIG_PREFIX.into()).unwrap();
    let s: k256::ecdsa::Signature = sk.sign_digest(h);
    Sig(s.as_bytes().into())
}

pub fn fully_sign(state: &State, keys: &[SigningKey]) -> Vec<Sig> {
    keys.iter().map(|key| sign(state, key)).collect()
}
