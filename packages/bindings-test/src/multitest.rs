use anyhow::{bail, Result as AnyResult};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::iter;
use std::ops::{Deref, DerefMut};
use thiserror::Error;

use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::{
    coins, to_binary, Addr, Api, BankMsg, Binary, BlockInfo, Coin, CustomQuery, Decimal, Empty,
    Fraction, Isqrt, Querier, QuerierResult, StdError, StdResult, Storage, Uint128,
};
use cw_multi_test::{
    App, AppResponse, BankKeeper, BankSudo, BasicAppBuilder, CosmosRouter, Module, WasmKeeper,
};
use cw_storage_plus::Map;

use crate::error::ContractError;
use jackal_bindings::{JackalMsg, JackalQuerier, JackalQuery};

impl Module for JackalModule {
    type ExecT = JackalMsg;

    // Builds a mock rust implementation of the expected jackal functionality for testing
    fn execute<ExecC, QueryC>(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn JackalRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        sender: Addr,
        msg: JackalMsg,
    ) -> AnyResult<AppResponse>
    where
        ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        match msg {
            JackalMsg::PostKey { key } => {
                let key = "placeholder";
                Ok(AppResponse {
                    data,
                    events: vec![],
                })
            },
            JackalMsg::MakeRoot { editors, viewers, trackingnumber } => {
                let editors = "placeholder";
                let viewers = "placeholder";
                let trackingnumber = "placeholder";

                Ok(AppResponse {
                    data,
                    events: vec![],
                })
            },

        }
    }
}

pub struct JackalModule {}

/// How many seconds per block
/// (when we increment block.height, use this multiplier for block.time)
pub const BLOCK_TIME: u64 = 5;

impl JackalModule {

}