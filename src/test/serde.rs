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

use crate::{test::common::random::*, types::*};

#[test]
fn state_serde() {
    let mut rng = rand::thread_rng();

    for _ in 0..99 {
        let state = random_state(&mut rng);

        let encoded: Vec<u8> = encode_obj(&state).unwrap();
        let decoded: State = decode_obj(&encoded).unwrap();

        assert_eq!(state, decoded);
    }
}

#[test]
fn state_params() {
    let mut rng = rand::thread_rng();

    for _ in 0..99 {
        let (params, _) = random_params(&mut rng);

        let encoded: Vec<u8> = encode_obj(&params).unwrap();
        let decoded: Params = decode_obj(&encoded).unwrap();

        assert_eq!(params, decoded);
    }
}

#[test]
fn state_withdrawal() {
    let mut rng = rand::thread_rng();

    for _ in 0..99 {
        let (withdrawal, _, _) = random_withdrawal(&mut rng);

        let encoded: Vec<u8> = encode_obj(&withdrawal).unwrap();
        let decoded: Withdrawal = decode_obj(&encoded).unwrap();

        assert_eq!(withdrawal, decoded);
    }
}
