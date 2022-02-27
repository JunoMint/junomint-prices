use cosmwasm_std::{BankMsg, Coin, CosmosMsg, Deps, MessageInfo, QueryRequest, Response, StdResult, SubMsg, to_binary, Uint128, WasmQuery};
use wasmswap::msg::Token1ForToken2PriceResponse;
use crate::ContractError;
use crate::error::PaymentError;
use crate::query::SwapDetailsResponse;
use crate::state::{SWAP_DETAILS, SwapDetails};

pub fn coin_amount(info: &MessageInfo, amount: Uint128) -> Result<Coin, PaymentError> {
    match info.funds.len() {
        0 => Err(PaymentError::NoFunds {}),
        1 => {
            let coin = &info.funds[0];
            if coin.amount.is_zero() {
                Err(PaymentError::NoFunds {})
            } else if coin.amount < amount {
                Err(PaymentError::NoEnoughFunds {})
            } else {
                Ok(coin.clone())
            }
        }
        _ => Err(PaymentError::MultipleDenoms {}),
    }
}

pub fn must_pay_amount(info: &MessageInfo, denom: &str, amount: Uint128) -> Result<Uint128, PaymentError> {
    let coin = coin_amount(info, amount)?;
    if coin.denom != denom {
        Err(PaymentError::MissingDenom(denom.to_string()))
    } else {
        Ok(coin.amount)
    }
}

pub fn convert_to_swap_details_response(
    swap_details: SwapDetails
) -> StdResult<SwapDetailsResponse> {
    Ok(SwapDetailsResponse{
        name: swap_details.name,
        receiver: swap_details.receiver,
        swap_address: swap_details.swap_address,
        token1_amount: swap_details.token1_amount,
        code_id: swap_details.code_id,
    })
}

pub fn send_payment(sender: String, receiver: String, coin: Vec<Coin>) -> Result<Response, ContractError> {
    let send_msg = SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
        to_address: receiver.clone(),
        amount: coin
    }));
    Ok(Response::new()
        .add_submessages(vec![send_msg])
        .add_attribute("action", "execute")
        .add_attribute("owner", sender)
    )
}

pub fn query_contract_price(
    deps: Deps,
    contract_addr: String,
    token1_amount: Uint128
) -> Token1ForToken2PriceResponse {
    deps.querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr,
            msg: to_binary(&wasmswap::msg::QueryMsg::Token1ForToken2Price {
                token1_amount
            }).unwrap(),
        })).unwrap()
}

pub fn query_code_price(deps: Deps, code_id: u64) -> Token1ForToken2PriceResponse {
    let swap_details: SwapDetails = SWAP_DETAILS.load(deps.storage, &code_id.to_string()).unwrap();
    query_contract_price(deps, swap_details.swap_address, swap_details.token1_amount)
}