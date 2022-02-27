use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct EmptyInstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SwapDetails {
        name: String,
        receiver: String,
        swap_address: String,
        token1_amount: Uint128,
        type_code: String
    }
}
