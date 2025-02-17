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

/// SwapPermit2GetPrice200ResponseAnyOfIssuesBalance : Details of balance of the `sellToken` that the `taker` must hold. Null if the `taker` has sufficient balance
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwapPermit2GetPrice200ResponseAnyOfIssuesBalance {
    /// The contract address of the `sellToken`
    #[serde(rename = "token")]
    pub token: String,
    /// The current balance of the `sellToken` in the `taker` address
    #[serde(rename = "actual")]
    pub actual: Box<serde_json::Value>,
    /// The balance of the `sellToken` required for the swap to execute successfully
    #[serde(rename = "expected")]
    pub expected: Box<serde_json::Value>,
}

impl SwapPermit2GetPrice200ResponseAnyOfIssuesBalance {
    /// Details of balance of the `sellToken` that the `taker` must hold. Null if the `taker` has sufficient balance
    pub fn new(token: String, actual: serde_json::Value, expected: serde_json::Value) -> SwapPermit2GetPrice200ResponseAnyOfIssuesBalance {
        SwapPermit2GetPrice200ResponseAnyOfIssuesBalance {
            token,
            actual: Box::new(actual),
            expected: Box::new(expected),
        }
    }
}

