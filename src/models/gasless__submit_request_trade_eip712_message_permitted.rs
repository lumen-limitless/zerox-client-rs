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
pub struct GaslessSubmitRequestTradeEip712MessagePermitted {
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "amount", deserialize_with = "Option::deserialize")]
    pub amount: Option<Box<models::GaslessSubmitRequestTradeEip712MessagePermittedAmount>>,
}

impl GaslessSubmitRequestTradeEip712MessagePermitted {
    pub fn new(token: String, amount: Option<models::GaslessSubmitRequestTradeEip712MessagePermittedAmount>) -> GaslessSubmitRequestTradeEip712MessagePermitted {
        GaslessSubmitRequestTradeEip712MessagePermitted {
            token,
            amount: if let Some(x) = amount {Some(Box::new(x))} else {None},
        }
    }
}

