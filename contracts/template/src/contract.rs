#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    CosmosMsg, WasmMsg, Empty, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::TemplateError;
use crate::msg::{InstantiateMsg, ExecuteTemplateMsg};
use crate::state::{State, STATE};
use jackal_bindings::{JackalMsg, JackalQuerier, JackalQuery};
use filetree::msg::{ExecuteMsg};

// Consider adding migration info?

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<JackalQuery>,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, TemplateError> {
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
    msg: ExecuteTemplateMsg,
) -> Result<Response, TemplateError> {
    match msg {
        ExecuteTemplateMsg::HitFiletreeMakeRoot {

        } => hit_file_tree_make_root(deps,info),
    }
}

// Call make_root in filetree contract 
pub fn hit_file_tree_make_root(
    deps: DepsMut<JackalQuery>,
    info: MessageInfo,
) -> Result<Response, TemplateError> {
    // TO DO: properly validate
    // deps.api.addr_validate(info.sender)?;

    // Checks and validations go here?
    let msg = ExecuteMsg::MakeRoot { 
        editors: "placeholder".to_string(), 
        viewers: "placeholder".to_string(), 
        trackingnumber: "placeholder".to_string() };
    
    let binary_msg = to_binary(&msg)?;

    let cosmos_msg: CosmosMsg<Empty> = CosmosMsg::from(WasmMsg::Execute {
        contract_addr: "jkl14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9scsc9nr".to_string(),
        funds: vec![],
        msg: binary_msg,
    });

    Ok(Response::new()
    .add_attribute("method", "hit filetree make root")
    .add_message(cosmos_msg)
    )
}
