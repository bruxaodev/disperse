use cosmwasm_std::{Addr, Coin, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Disperse {
        accounts: Vec<Addr>,
        amounts: Vec<Uint128>,
    },
    DisperseSameValue {
        accounts: Vec<Addr>,
        amount: Uint128,
    },
    WithdrawFunds {
        accounts: Vec<Addr>,
        amounts: Vec<Coin>,
    },
    UpdateAdmin {
        new_admin: Addr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}
