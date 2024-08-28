// #![cfg(test)]

// use cosmwasm_schema::cw_serde;
// use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};

// use crate::msg::ExecuteMsg;
// use crate::msg::SudoMsg;

// /// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
// /// for working with this.
// #[cw_serde]
// pub struct RateLimitingContract(pub Addr);

// impl RateLimitingContract {
//     pub fn addr(&self) -> Addr {
//         self.0.clone()
//     }

//     pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
//         let msg = to_binary(&msg.into())?;
//         Ok(WasmMsg::Execute {
//             contract_addr: self.addr().into(),
//             msg,
//             funds: vec![],
//         }
//         .into())
//     }

//     pub fn sudo<T: Into<SudoMsg>>(&self, msg: T) -> cw_multi_test::SudoMsg {
//         let msg = to_binary(&msg.into()).unwrap();
//         cw_multi_test::SudoMsg::Wasm(cw_multi_test::WasmSudo {
//             contract_addr: self.addr().into(),
//             msg,
//         })
//     }
// }

// pub mod tests {
//     use cosmwasm_std::{Timestamp, Uint128};

//     use crate::state::RateLimit;

//     pub fn verify_query_response(
//         value: &RateLimit,
//         quota_name: &str,
//         send_recv: (u32, u32),
//         duration: u64,
//         inflow: Uint128,
//         outflow: Uint128,
//         period_end: Timestamp,
//     ) {
//         assert_eq!(value.quota.name, quota_name);
//         assert_eq!(value.quota.max_percentage_send, send_recv.0);
//         assert_eq!(value.quota.max_percentage_recv, send_recv.1);
//         assert_eq!(value.quota.duration, duration);
//         assert_eq!(value.flow.inflow, inflow);
//         assert_eq!(value.flow.outflow, outflow);
//         assert_eq!(value.flow.period_end, period_end);
//     }
// }
