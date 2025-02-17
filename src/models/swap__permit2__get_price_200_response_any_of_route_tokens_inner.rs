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

/// SwapPermit2GetPrice200ResponseAnyOfRouteTokensInner : Properties of the tokens involved in the swap
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwapPermit2GetPrice200ResponseAnyOfRouteTokensInner {
    /// The token address. This is the unique identifier of the token
    #[serde(rename = "address")]
    pub address: String,
    /// The token symbol. This is not guaranteed to be unique, as multiple tokens can have the same symbol
    #[serde(rename = "symbol")]
    pub symbol: String,
}

impl SwapPermit2GetPrice200ResponseAnyOfRouteTokensInner {
    /// Properties of the tokens involved in the swap
    pub fn new(address: String, symbol: String) -> SwapPermit2GetPrice200ResponseAnyOfRouteTokensInner {
        SwapPermit2GetPrice200ResponseAnyOfRouteTokensInner {
            address,
            symbol,
        }
    }
}

