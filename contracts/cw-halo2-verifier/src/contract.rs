use crate::error::ContractError;
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw_halo2::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::*,
};

/// Name of the plugin
/// This must be the same name as it is registered at the plugin registry
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
/// Version of the plugin
/// This must be the same version as it is registered at the plugin registry
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // Depending on the plugin this may or may not be supported
    unimplemented!()
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // Depending on the plugin this may or may not be supported
    unimplemented!()
}
