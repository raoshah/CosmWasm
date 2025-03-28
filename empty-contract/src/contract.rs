use crate::msg::{GreetResp, QueryMsg};
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};

pub fn instantiate( _deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty,) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;
    match _msg {
        Greet {} => to_json_binary(&query::greet()?),
    } 
}

mod query {
    use super::*;

    pub fn greet() -> StdResult<GreetResp> {
        let resp = GreetResp{ message: "Hello World".to_owned(),};
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use super::*;

    #[test]
    fn greet_query() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        instantiate(
            deps.as_mut(),
            env.clone(),
            mock_info("sender", &[]),
            Empty {},
        ).unwrap();
        let resp = query(deps.as_ref(), env, QueryMsg::Greet {}).unwrap();
        let resp = query(deps.as_ref(), env, QueryMsg::Greet {}).unwrap();
        let resp: GreetResp = from_binary(&resp).unwrap();
        assert_eq!(resp, GreetResp {message: "Hello World".to_owned()});
    }
}