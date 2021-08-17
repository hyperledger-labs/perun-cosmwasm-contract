#[path = "../tests/common/random.rs"]
mod random;

use cosmwasm_schema::remove_schemas;
use perun_cosmwasm::types::encode_obj;
use rand::{rngs::StdRng, SeedableRng};
use serde::Serialize;
use std::{
    env::current_dir,
    fs::{create_dir_all, remove_file, write},
    path::Path,
};

/// Entry point for generating binary files for testing the go-perun connector.
fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("serde");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    let mut rng = StdRng::seed_from_u64(1234);
    let (params, state) = random::random_state(&mut rng);

    write_obj(&params, "params", &out_dir);
    write_obj(&state, "state", &out_dir);
}

fn write_obj<T: Serialize>(obj: &T, name: &str, dir: &Path) {
    let data = encode_obj(obj).unwrap();
    let path = dir.join(format!("{}.bin", name));
    let _ = remove_file(&path);
    write(&path, data).expect("Could not write data to file");
}
