use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, DepsMut, MessageInfo};
use cw_storage_plus::Item;

use crate::error::ContractError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub admin: Addr,
}

impl State {
    fn is_admin(&self, sender: &Addr) -> bool {
        sender == &self.admin
    }

    pub fn only_admin(&self, sender: &Addr) -> Result<(), ContractError> {
        if !self.is_admin(sender) {
            return Err(ContractError::Unauthorized {});
        }
        Ok(())
    }

    pub fn set_admin(&mut self,_deps: DepsMut, _info: MessageInfo, new_admin: &Addr) -> Result<(), ContractError> {
        self.only_admin(&_info.sender)?;
        let mut state= STATE.load(_deps.storage)?;
        state.admin =new_admin.clone();
        STATE.save(_deps.storage, &state)?;
        Ok(())
    }
}

pub const STATE: Item<State> = Item::new("state");
