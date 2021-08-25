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

use cosmwasm_schema::remove_schemas;
use perun_cosmwasm::{
    test::common::random,
    types::{encode_obj, Funding},
};
use rand::{rngs::StdRng, SeedableRng};
use serde::Serialize;
use serde_json::json;
use std::{
    env::current_dir,
    fs::{create_dir_all, remove_file, write},
    path::Path,
    string::String,
};

/// Entry point for generating binary files for testing the go-perun connector.
fn main() {
    let mut dir = current_dir().unwrap();
    dir.push("serde");
    create_dir_all(&dir).unwrap();
    let mut rng = StdRng::seed_from_u64(1234);

    // Write encoding examples.
    let state = random::random_state(&mut rng);
    write_obj(&state, "state", &dir);
    let (params, _) = random::random_params(&mut rng);
    write_obj(&params, "params", &dir);
    let (withdrawal, _, _) = random::random_withdrawal(&mut rng);
    write_obj(&withdrawal, "withdrawal", &dir);
    let funding = Funding {
        channel: withdrawal.channel_id.clone(),
        part: withdrawal.part.clone(),
    };
    write_obj(&funding, "funding", &dir);

    // Write ChannelID and FundingID examples.
    let json = json!(
    {
        "__comment": "The channel_id is calculated from params.bin and the funding_id from withdrawal.bin with the part_index",
        "channel_id": hex::encode(params.channel_id().unwrap()),
        "funding_id": hex::encode(withdrawal.funding_id().unwrap()),
        "part": hex::encode(withdrawal.part.0),
    })
    .to_string();
    write_file(json.into(), "ids.json".into(), &dir);
}

fn write_obj<T: Serialize>(obj: &T, name: &str, dir: &Path) {
    let data = encode_obj(obj).unwrap();
    write_file(data, format!("{}.bin", name), dir);
}

fn write_file(data: Vec<u8>, name: String, dir: &Path) {
    let path = dir.join(name);
    let _ = remove_file(&path);
    write(&path, data).expect("Could not write data to file");
}
