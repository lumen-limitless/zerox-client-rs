/*
 * 0x API
 *
 * These are the API references for the beta version of 0x API v2. If you are looking for API v1 references, [see here](/0x-swap-api/api-references/overview).
 *
 * The version of the OpenAPI document: next
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GaslessSubmit400Response {
    #[serde(rename = "name")]
    pub name: Name,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "data")]
    pub data: Box<models::PendingTradesAlreadyExistData>,
}

impl GaslessSubmit400Response {
    pub fn new(name: Name, message: String, data: models::PendingTradesAlreadyExistData) -> GaslessSubmit400Response {
        GaslessSubmit400Response {
            name,
            message,
            data: Box::new(data),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "PENDING_TRADES_ALREADY_EXIST")]
    PendingTradesAlreadyExist,
}

impl Default for Name {
    fn default() -> Name {
        Self::PendingTradesAlreadyExist
    }
}

