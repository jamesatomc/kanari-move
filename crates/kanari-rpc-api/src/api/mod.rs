// Copyright (c) Kanari Network
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::RpcModule;

pub mod btc_api;
pub mod kanari_api;

pub const DEFAULT_RESULT_LIMIT: u64 = 50;
pub const DEFAULT_RESULT_LIMIT_USIZE: usize = DEFAULT_RESULT_LIMIT as usize;

pub const MAX_RESULT_LIMIT: u64 = 200;
pub const MAX_RESULT_LIMIT_USIZE: usize = MAX_RESULT_LIMIT as usize;
pub const MAX_INTERNAL_LIMIT_USIZE: usize = 2000;

// pub fn validate_limit(limit: Option<u64>, max: usize) -> Result<usize, anyhow::Error> {
//     match limit {
//         Some(l) if l > max => Err(anyhow!("Page size limit {l} exceeds max limit {max}")),
//         Some(0) => Err(anyhow!("Page size limit cannot be smaller than 1")),
//         Some(l) => Ok(l),
//         None => Ok(max),
//     }
// }

pub trait KanariRpcModule
where
    Self: Sized,
{
    fn rpc(self) -> RpcModule<Self>;
}
