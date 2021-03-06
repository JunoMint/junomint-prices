use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, StdResult, Uint128};
use cw_storage_plus::{Item, Map};
use crate::query::SwapDetailsResponse;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SwapDetails {
    pub name: String,
    pub receiver: String,
    pub swap_address: String,
    pub token2_amount: Uint128,
    pub type_code: String
}

impl SwapDetails {
    pub fn convert_to_response(&self) -> StdResult<SwapDetailsResponse> {
        let swap_details = self.clone();
        Ok(SwapDetailsResponse{
            name: swap_details.name,
            receiver: swap_details.receiver,
            swap_address: swap_details.swap_address,
            token2_amount: swap_details.token2_amount,
            type_code: swap_details.type_code,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Admin {
    pub address: Addr
}

pub const SWAP_DETAILS: Map<&str, SwapDetails> = Map::new("swap_details");
pub const ADMIN: Item<Admin> = Item::new("admin");
