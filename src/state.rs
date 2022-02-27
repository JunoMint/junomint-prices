use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{StdResult, Uint128};
use cw_storage_plus::{Map};
use crate::query::SwapDetailsResponse;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SwapDetails {
    pub name: String,
    pub receiver: String,
    pub swap_address: String,
    pub token1_amount: Uint128,
    pub code_id: u64
}

impl SwapDetails {
    pub fn convert_to_response(&self) -> StdResult<SwapDetailsResponse> {
        let swap_details = self.clone();
        Ok(SwapDetailsResponse{
            name: swap_details.name,
            receiver: swap_details.receiver,
            swap_address: swap_details.swap_address,
            token1_amount: swap_details.token1_amount,
            code_id: swap_details.code_id,
        })
    }
}

pub const SWAP_DETAILS: Map<&str, SwapDetails> = Map::new("swap_details");
