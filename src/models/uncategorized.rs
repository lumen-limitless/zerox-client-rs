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
pub struct Uncategorized {
    #[serde(rename = "name")]
    pub name: Name,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "data")]
    pub data: Box<models::BuyTokenNotAuthorizedForTradeData>,
}

impl Uncategorized {
    pub fn new(name: Name, message: String, data: models::BuyTokenNotAuthorizedForTradeData) -> Uncategorized {
        Uncategorized {
            name,
            message,
            data: Box::new(data),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "UNCATEGORIZED")]
    Uncategorized,
}

impl Default for Name {
    fn default() -> Name {
        Self::Uncategorized
    }
}

