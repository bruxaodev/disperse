use std::ops::Sub;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Addr, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{self, State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:disperse";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state: State = State {
        admin: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Disperse { accounts, amounts } => disperse(deps, info, accounts, amounts),
        ExecuteMsg::DisperseSameValue { accounts, amount } => {
            disperse_same_value(deps, info, accounts, amount)
        }
        ExecuteMsg::WithdrawFunds { amount } => withdraw_funds(deps, info, amount),
    }
}

pub fn disperse(
    deps: DepsMut,
    info: MessageInfo,
    accounts: Vec<Addr>,
    amounts: Vec<Coin>,
) -> Result<Response, ContractError> {
    if accounts.len() != amounts.len() {
        return Err(ContractError::InvalidInput {
            reason: "Number of accounts and amounts do not match".to_string(),
        });
    }
    let mut total_amount_to_disperse = Uint128::zero();
    let mut messages: Vec<CosmosMsg> = Vec::new();
    for (account, amount) in accounts.into_iter().zip(amounts.into_iter()) {
        total_amount_to_disperse += amount.amount;
        let msg: BankMsg = BankMsg::Send {
            to_address: account.into_string(),
            amount: vec![amount],
        };
        let cosmos_msg: CosmosMsg = CosmosMsg::Bank(msg);
        messages.push(cosmos_msg);
    }
    let response: Response = Response::new()
        .add_messages(messages)
        .add_attribute("action", "disperse");

    Ok(response)
}

pub fn disperse_same_value(
    deps: DepsMut,
    info: MessageInfo,
    accounts: Vec<Addr>,
    amount: Coin,
) -> Result<Response, ContractError> {
    let mut total_amount_to_disperse = Uint128::zero();
    let mut messages: Vec<CosmosMsg> = Vec::new();
    for account in accounts {
        total_amount_to_disperse += amount.amount;
        let msg: BankMsg = BankMsg::Send {
            to_address: account.into_string(),
            amount: vec![amount.clone()],
        };
        let cosmos_msg: CosmosMsg = CosmosMsg::Bank(msg);
        messages.push(cosmos_msg);
    }

    let response: Response = Response::new()
        .add_messages(messages)
        .add_attribute("action", "disperse");

    Ok(response)
}

pub fn withdraw_funds(
    deps: DepsMut,
    info: MessageInfo,
    amount: Vec<BankMsg>,
) -> Result<Response, ContractError> {
    let state: State = STATE.load(deps.storage)?;
    if info.sender != state.admin {
        return Err(ContractError::Unauthorized {});
    }

    let response: Response = Response::new()
        .add_messages(amount)
        .add_attribute("action", "withdraw_funds");
    Ok(response)
}