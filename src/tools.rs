use cosmwasm_std::{BankMsg, Coin, CosmosMsg, Deps, MessageInfo, QueryRequest, Response, StdError, StdResult, SubMsg, to_binary, Uint128, WasmQuery};
use wasmswap::msg::{Token2ForToken1PriceResponse};
use crate::{query};
use crate::error::PaymentError;
use crate::query::{SwapDetailsPriceResponse, SwapDetailsResponse};

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
    token2_amount: Uint128
) -> StdResult<Token2ForToken1PriceResponse> {
    let mut token1: Token2ForToken1PriceResponse = deps.querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr,
            msg: to_binary(&wasmswap::msg::QueryMsg::Token2ForToken1Price {
                token2_amount
            })?,
        }))?;

    token1.token1_amount = token1.token1_amount
        .checked_div(Uint128::new(97))
        .map_err(StdError::divide_by_zero)?
        .checked_mul(Uint128::new(100))// LP Fees
        .map_err(StdError::overflow)?;

    Ok(token1)
}

pub fn query_code_price(
    deps: Deps,
    contract_addr: String,
    type_code: String
) -> StdResult<SwapDetailsPriceResponse> {
    let details: SwapDetailsResponse = deps.querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr,
            msg: to_binary(&query::QueryMsg::SwapDetails {
                type_code: type_code.clone()
            })?,
        }))?;
    let price: Token2ForToken1PriceResponse = query_contract_price(
        deps,
        details.swap_address.clone(),
        details.token2_amount
    )?;
    Ok(SwapDetailsPriceResponse {
        name: details.name,
        receiver: details.receiver,
        swap_address: details.swap_address,
        token1_amount: price.token1_amount,
        token2_amount: details.token2_amount,
        type_code
    })
}