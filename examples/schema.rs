use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use wasmswap::msg::Token1ForToken2PriceResponse;

use cw20_instantiator::query::{QueryMsg, SwapDetailsResponse};
use cw20_instantiator::msg::{ExecuteMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(SwapDetailsResponse), &out_dir);
    export_schema(&schema_for!(Token1ForToken2PriceResponse), &out_dir);
}
