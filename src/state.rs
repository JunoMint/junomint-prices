use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint128};
use cw_storage_plus::{Map};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SwapDetails {
    pub name: String,
    pub receiver: String,
    pub swap_address: String,
    pub token1_amount: Uint128,
    pub code_id: u64
}

pub const SWAP_DETAILS: Map<&str, SwapDetails> = Map::new("swap_details");
