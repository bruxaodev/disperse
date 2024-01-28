use std::ops::Sub;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:dispersei";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state: State = State {
        admin: _info.sender.clone(),
    };
    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(_deps.storage, &state)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match _msg {
        ExecuteMsg::Disperse { accounts, amounts } => disperse(_info, accounts, amounts),
        ExecuteMsg::DisperseSameValue { accounts, amount } => {
            disperse_same_value(_info, accounts, amount)
        }
        ExecuteMsg::WithdrawFunds { accounts, amounts } => withdraw_funds(_deps, _info, accounts, amounts),
        ExecuteMsg::UpdateAdmin { new_admin } => update_admin(_info, _deps, new_admin),
    }
}

pub fn disperse(
    _info: MessageInfo,
    accounts: Vec<Addr>,
    amounts: Vec<Uint128>,
) -> Result<Response, ContractError> {
    if accounts.len() != amounts.len() {
        return Err(ContractError::InvalidInput {
            reason: "Number of accounts and amounts do not match".to_string(),
        });
    }

    let mut messages: Vec<CosmosMsg> = Vec::new();
    for (account, amount) in accounts.into_iter().zip(amounts.clone().into_iter()) {
        let msg: BankMsg = BankMsg::Send {
            to_address: account.into_string(),
            amount: vec![Coin::new(amount.u128(), _info.funds[0].denom.clone())]
        };
        let cosmos_msg: CosmosMsg = CosmosMsg::Bank(msg);
        messages.push(cosmos_msg);
    }


    let total_amount_to_disperse: Uint128 = amounts.clone().iter().sum();
    let remaining = Uint128::new(_info.funds[0].amount.u128()).sub(total_amount_to_disperse);
    if !remaining.is_zero() {
        let refund = BankMsg::Send {
            to_address: _info.sender.into_string(),
            amount: vec![Coin {
                denom: _info.funds[0].denom.clone(),
                amount: remaining,
            }],
        };

        messages.push(CosmosMsg::Bank(refund))
    }

    let response: Response = Response::new()
        .add_messages(messages)
        .add_attribute("action", "disperse");

    Ok(response)
}

pub fn disperse_same_value(
    _info: MessageInfo,
    accounts: Vec<Addr>,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = Vec::new();
    let mut total_amount_to_disperse: Uint128 = Uint128::zero();
    for account in accounts {
        total_amount_to_disperse += amount;
        let msg: BankMsg = BankMsg::Send {
            to_address: account.into_string(),
            amount: vec![Coin{
                denom: _info.funds[0].denom.clone(),
                amount: amount
            }]
        };
        let cosmos_msg: CosmosMsg = CosmosMsg::Bank(msg);
        messages.push(cosmos_msg);
    }

    let remaining = Uint128::new(_info.funds[0].amount.u128()).sub(total_amount_to_disperse);
    if !remaining.is_zero() {
        let refund = BankMsg::Send {
            to_address: _info.sender.into_string(),
            amount: vec![Coin {
                denom: _info.funds[0].denom.clone(),
                amount: remaining,
            }],
        };

        messages.push(CosmosMsg::Bank(refund))
    }

    let response: Response = Response::new()
        .add_messages(messages)
        .add_attribute("action", "disperse");

    Ok(response)
}

pub fn withdraw_funds(
    deps: DepsMut,
    info: MessageInfo,
    accounts: Vec<Addr>,
    amounts: Vec<Coin>,
) -> Result<Response, ContractError> {
    let state: State = STATE.load(deps.storage)?;
    state.only_admin(&info.sender)?;
    let mut messages: Vec<CosmosMsg> = Vec::new();
    for (account, amount) in accounts.into_iter().zip(amounts.into_iter()){
        let msg = BankMsg::Send {
            to_address: account.into_string(),
            amount: vec![amount]
        };
        messages.push(CosmosMsg::Bank(msg))
    }

    let response: Response = Response::new()
        .add_messages(messages)
        .add_attribute("action", "withdraw_funds");
    Ok(response)
}

pub fn update_admin(
    _info: MessageInfo,
    deps: DepsMut,
    new_admin: Addr,
) -> Result<Response, ContractError> {
    let mut state: State = STATE.load(deps.storage)?;
    state.set_admin(deps, _info, &new_admin)?;
    Ok(Response::default())
}
