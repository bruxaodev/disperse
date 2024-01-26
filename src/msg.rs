use cosmwasm_std::{Addr, Coin, BankMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Disperse {
        accounts: Vec<Addr>,
        amounts: Vec<Coin>,
    },
    DisperseSameValue {
        accounts: Vec<Addr>,
        amount: Coin,
    },
    WithdrawFunds {
        amount: Vec<BankMsg>,
    },
    UpdateAdmin {
        new_admin: Addr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}
