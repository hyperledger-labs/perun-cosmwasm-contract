use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use perun_cosmwasm::msg::*;
use std::{env::current_dir, fs::create_dir_all};

/// Entry point for generating the schema files.
fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InitMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
}
