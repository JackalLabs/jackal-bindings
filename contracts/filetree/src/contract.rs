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
        ExecuteMsg::PostKey {
            key,

        } => post_key(deps,info, key),
        ExecuteMsg::MakeRoot {
            editors,
            viewers,
            // TO DO
            // Use UUID library to populate trackingnumber inside make_root() so 
            // we don't need to pass it into canined or a ts client?
            trackingnumber,  
        // MessageInfo.sender is the creator of the root file
        } => make_root(deps,info, editors, viewers, trackingnumber),
        ExecuteMsg::PostFiles {
            account,
            hashparent,
            hashchild,
            contents,  
            viewers,
            editors,
            trackingnumber,
        // MessageInfo.sender is the creator of the root file
        } => post_files(deps, info, account, hashparent, hashchild, contents, viewers, editors, trackingnumber),
        ExecuteMsg::DeleteFile {
            hashpath,
            account,
        } => delete_file(deps, info, hashpath, account),
        ExecuteMsg::BuyStorage {
            foraddress,
            duration,
            bytes,
            paymentdenom,
        } => buy_storage(deps, info, foraddress, duration, bytes, paymentdenom),
        ExecuteMsg::PostAndSign {
            account,
            hashparent,
            hashchild,
            contents,  
            viewers,
            editors,
            trackingnumber,
            cid,
            payonce,
        } => post_and_sign(deps, info, account, hashparent, hashchild, contents, viewers, editors, trackingnumber, cid, payonce),
        ExecuteMsg::DeleteAndCancel {
            hashpath,
            account,
            cids,
        } => delete_and_cancel(deps, info, hashpath, account, cids),
    }
}

pub fn post_key(
    deps: DepsMut<JackalQuery>,
    info: MessageInfo,
    key: String,
) -> Result<Response<JackalMsg>, FiletreeError> {
    // TO DO
    // properly validate
    // deps.api.addr_validate(info.sender)?;

    // Checks and validations go here?
    let post_key_msg = JackalMsg::post_key(key);

    let res = Response::new()
        .add_attribute("method", "post_key")
        .add_message(post_key_msg);

    Ok(res)
}

pub fn make_root(
    deps: DepsMut<JackalQuery>,
    info: MessageInfo,
    editors: String,
    viewers: String,
    trackingnumber: String,
) -> Result<Response<JackalMsg>, FiletreeError> {
    // TO DO
    // properly validate
    // deps.api.addr_validate(info.sender)?;

    // Checks and validations go here?
    let make_root_msg = JackalMsg::make_root(editors, viewers,trackingnumber );

    let res = Response::new()
        .add_attribute("method", "make_root")
        .add_message(make_root_msg);

    Ok(res)
}

pub fn post_files(
    deps: DepsMut<JackalQuery>,
    info: MessageInfo,
    account: String,
    hashparent: String,
    hashchild: String,
    contents: String,
    viewers: String,
    editors: String,
    trackingnumber: String,
) -> Result<Response<JackalMsg>, FiletreeError> {
    // TO DO
    //deps.api.addr_validate(&nfo.sender)?;

    // Checks and validations go here?
    let post_file_msg = JackalMsg::post_files(
        account, hashparent, hashchild,contents, viewers, editors,trackingnumber  );

    let res = Response::new()
        .add_attribute("method", "post_file")
        .add_message(post_file_msg);

    Ok(res)
}

pub fn delete_file(
    deps: DepsMut<JackalQuery>,
    info: MessageInfo,
    hashpath: String,
    account: String,
) -> Result<Response<JackalMsg>, FiletreeError> {
    // TO DO
    //deps.api.addr_validate(&nfo.sender)?;

    // Checks and validations go here?
    let delete_file_msg = JackalMsg::delete_file(
        hashpath, account);

    let res = Response::new()
        .add_attribute("method", "delete_file")
        .add_message(delete_file_msg);

    Ok(res)
}

pub fn buy_storage(
    deps: DepsMut<JackalQuery>,
    info: MessageInfo,
    foraddress: String,
    duration: String,
    bytes: String,
    paymentdenom: String
) -> Result<Response<JackalMsg>, FiletreeError> {
    // TO DO
    //deps.api.addr_validate(&nfo.sender)?;

    // Checks and validations go here?
    let buy_storage_msg = JackalMsg::buy_storage(
        foraddress, duration, bytes, paymentdenom);

    let res = Response::new()
        .add_attribute("method", "buy_storage")
        .add_message(buy_storage_msg);

    Ok(res)
}


pub fn post_and_sign(
    deps: DepsMut<JackalQuery>,
    info: MessageInfo,
    account: String,
    hashparent: String,
    hashchild: String,
    contents: String,
    viewers: String,
    editors: String,
    trackingnumber: String,
    cid: String,
    payonce: bool,
) -> Result<Response<JackalMsg>, FiletreeError> {
    // TO DO
    //deps.api.addr_validate(&nfo.sender)?;

    // Checks and validations go here?
    let post_and_sign_msg = JackalMsg::post_and_sign(
        account, hashparent, hashchild,contents, viewers, editors,trackingnumber, cid, payonce );

    let res = Response::new()
        .add_attribute("method", "post_and_sign")
        .add_message(post_and_sign_msg);

    Ok(res)
}

pub fn delete_and_cancel(
    deps: DepsMut<JackalQuery>,
    info: MessageInfo,
    hashpath: String,
    account: String,
    cids: String,
) -> Result<Response<JackalMsg>, FiletreeError> {
    // TO DO
    //deps.api.addr_validate(&nfo.sender)?;

    // Checks and validations go here?
    let delete_and_cancel_msg = JackalMsg::delete_and_cancel(
        hashpath, account, cids);

    let res = Response::new()
        .add_attribute("method", "delete_and_cancel")
        .add_message(delete_and_cancel_msg);

    Ok(res)
}

