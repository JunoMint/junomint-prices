use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapDetailsResponse {
    pub name: String,
    pub receiver: String,
    pub swap_address: String,
    pub token1_amount: Uint128,
    pub type_code: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Token1ForToken2Price {
        type_code: String
    },
    SwapDetails {
        type_code: String
    }
}
