use cosmwasm_std::{BankMsg, Coin, CosmosMsg, MessageInfo, Response, StdResult, SubMsg, Uint128};
use crate::error::PaymentError;
use crate::query::SwapDetailsResponse;
use crate::state::SwapDetails;

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

pub fn send_payment(sender: String, receiver: String, coin: Vec<Coin>) -> StdResult<Response> {
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
