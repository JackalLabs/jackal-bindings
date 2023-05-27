#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::FiletreeError;
use crate::msg::{ExecuteMsg, GetFilesResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use jackal_bindings::{JackalMsg, JackalQuerier, JackalQuery};

// Consider adding migration info?

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<JackalQuery>,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, FiletreeError> {
    let state = State {
        owner: info.sender.clone(),
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<JackalQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<JackalMsg>, FiletreeError> {
    match msg {
        ExecuteMsg::MakeRoot {
            editors,
            viewers,
            // TO DO
            // Use UUID library to populate trackingnumber inside make_root() so 
            // we don't need to pass it into canined or a ts client?
            trackingnumber,  
        // MessageInfo.sender is the creator of the root file
        } => make_root(deps, info.sender.clone().into_string(), editors, viewers, trackingnumber)
    }
}

pub fn make_root(
    deps: DepsMut<JackalQuery>,
    sender: String,
    editors: String,
    viewers: String,
    trackingnumber: String,
) -> Result<Response<JackalMsg>, FiletreeError> {
    deps.api.addr_validate(&sender)?;

    // Checks and validations go here?
    let make_root_msg = JackalMsg::make_root(sender, editors, viewers,trackingnumber );

    let res = Response::new()
        .add_attribute("method", "make_root")
        .add_message(make_root_msg);

    Ok(res)
}

