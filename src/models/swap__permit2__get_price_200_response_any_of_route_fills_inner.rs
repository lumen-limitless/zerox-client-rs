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
pub struct SwapPermit2GetPrice200ResponseAnyOfRouteFillsInner {
    /// The contract address of the input token
    #[serde(rename = "from")]
    pub from: String,
    /// The contract address of the output token
    #[serde(rename = "to")]
    pub to: String,
    /// The liquidity source used in the route
    #[serde(rename = "source")]
    pub source: String,
    /// The proportion of the trade to be filled by the `source`
    #[serde(rename = "proportionBps")]
    pub proportion_bps: Box<serde_json::Value>,
}

impl SwapPermit2GetPrice200ResponseAnyOfRouteFillsInner {
    pub fn new(from: String, to: String, source: String, proportion_bps: serde_json::Value) -> SwapPermit2GetPrice200ResponseAnyOfRouteFillsInner {
        SwapPermit2GetPrice200ResponseAnyOfRouteFillsInner {
            from,
            to,
            source,
            proportion_bps: Box::new(proportion_bps),
        }
    }
}

