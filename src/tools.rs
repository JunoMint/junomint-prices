use cosmwasm_std::{BankMsg, Coin, CosmosMsg, Deps, MessageInfo, QueryRequest, Response, StdResult, SubMsg, to_binary, Uint128, WasmQuery};
use wasmswap::msg::Token1ForToken2PriceResponse;
use crate::{query};
use crate::error::PaymentError;
use crate::query::SwapDetailsResponse;

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

pub fn send_payment(sender: String, receiver: String, coin: Vec<Coin>) -> Response<> {
    let send_msg = SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
        to_address: receiver.clone(),
        amount: coin
    }));
    Response::new()
        .add_submessages(vec![send_msg])
        .add_attribute("action", "execute")
        .add_attribute("owner", sender)
}

pub fn query_contract_price(
    deps: Deps,
    contract_addr: String,
    token1_amount: Uint128
) -> StdResult<Token1ForToken2PriceResponse> {
    Ok(deps.querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr,
            msg: to_binary(&wasmswap::msg::QueryMsg::Token1ForToken2Price {
                token1_amount
            })?,
        }))?)
}

pub fn query_code_price(
    deps: Deps,
    contract_addr: String,
    type_code: String
) -> StdResult<SwapDetailsResponse> {
    let details: SwapDetailsResponse = deps.querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr,
            msg: to_binary(&query::QueryMsg::SwapDetails {
                type_code: type_code.clone()
            })?,
        }))?;
    let price: Token1ForToken2PriceResponse = query_contract_price(
        deps,
        details.swap_address.clone(),
        details.token1_amount
    )?;
    Ok(SwapDetailsResponse {
        name: details.name,
        receiver: details.receiver,
        swap_address: details.swap_address,
        token1_amount: price.token2_amount,
        type_code
    })
}