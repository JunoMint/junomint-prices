use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128
};

use cw2::set_contract_version;

use wasmswap::msg::{Token1ForToken2PriceResponse};

use crate::error::ContractError;
use crate::msg::{EmptyInstantiateMsg, ExecuteMsg};
use crate::query::{QueryMsg, SwapDetailsResponse};
use crate::state::{SwapDetails, SWAP_DETAILS};
use crate::tools;

const CONTRACT_NAME: &str = "crates.io:junomint-prices";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: EmptyInstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

pub fn execute_set_swap_details(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    name: String,
    receiver: String,
    swap_address: String,
    token1_amount: Uint128,
    code_id: u64
) -> Result<Response, ContractError> {
    let swap_details: SwapDetails = SwapDetails {
        name,
        receiver,
        swap_address,
        token1_amount,
        code_id,
    };
    SWAP_DETAILS.save(deps.storage, &code_id.to_string(),&swap_details).ok();
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SwapDetails {
            name,
            receiver,
            swap_address,
            token1_amount,
            code_id
        }
        => {
            Ok(execute_set_swap_details(
                deps,
                env,
                info,
                name,
                receiver,
                swap_address,
                token1_amount,
                code_id,
            )?)
        },
    }
}

pub fn query_price(deps: Deps, code_id: u64) -> StdResult<Token1ForToken2PriceResponse> {
    let swap_details: SwapDetails = SWAP_DETAILS.load(deps.storage, &code_id.to_string())?;
    Ok(tools::query_contract_price(
        deps,
        swap_details.swap_address,
        swap_details.token1_amount
    )?)
}

pub fn query_swap_details(deps: Deps, code_id: u64) -> StdResult<SwapDetailsResponse> {
    let swap_details: SwapDetails = SWAP_DETAILS.load(
        deps.storage,
        &code_id.to_string()
    )?;
    swap_details.convert_to_response()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Token1ForToken2Price {code_id} => to_binary(&query_price(deps, code_id)?),
        QueryMsg::SwapDetails {code_id} => to_binary(&query_swap_details(deps, code_id)?)
    }
}
