use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, StdError};

use cw2::set_contract_version;

use wasmswap::msg::{Token2ForToken1PriceResponse};

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
    token2_amount: Uint128,
    type_code: String
) -> Result<Response, ContractError> {
    let swap_details = SwapDetails {
        name,
        receiver,
        swap_address,
        token2_amount,
        type_code: type_code.clone(),
    };
    SWAP_DETAILS.save(deps.storage, &type_code,&swap_details).ok();
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
            token2_amount,
            type_code
        }
        => {
            Ok(execute_set_swap_details(
                deps,
                env,
                info,
                name,
                receiver,
                swap_address,
                token2_amount,
                type_code,
            )?)
        },
    }
}

pub fn query_price(deps: Deps, type_code: String) -> StdResult<Token2ForToken1PriceResponse> {
    if !SWAP_DETAILS.has(deps.storage, &type_code) {
        return Err(StdError::generic_err("Details not found"))
    }
    let swap_details: SwapDetails = SWAP_DETAILS.load(
        deps.storage,
        &type_code
    )?;
    Ok(tools::query_contract_price(
        deps,
        swap_details.swap_address,
        swap_details.token2_amount
    )?)
}

pub fn query_swap_details(deps: Deps, type_code: String) -> StdResult<SwapDetailsResponse> {
    if !SWAP_DETAILS.has(deps.storage, &type_code) {
        return Err(StdError::generic_err("Details not found"))
    }
    let swap_details: SwapDetails = SWAP_DETAILS.load(
        deps.storage,
        &type_code
    )?;
    swap_details.convert_to_response()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Token1ForToken2Price { type_code } => to_binary(&query_price(deps, type_code)?),
        QueryMsg::SwapDetails {type_code} => to_binary(&query_swap_details(deps, type_code)?)
    }
}
