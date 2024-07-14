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

/// SwapPermit2GetPrice200ResponseAnyOfFees : Fees to be deducted in this transaction. It contains the `integratorFee`, `zeroExFee` and `gasFee`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwapPermit2GetPrice200ResponseAnyOfFees {
    #[serde(rename = "integratorFee", deserialize_with = "Option::deserialize")]
    pub integrator_fee: Option<Box<models::SwapPermit2GetPrice200ResponseAnyOfFeesIntegratorFee>>,
    #[serde(rename = "zeroExFee", deserialize_with = "Option::deserialize")]
    pub zero_ex_fee: Option<Box<models::SwapPermit2GetPrice200ResponseAnyOfFeesZeroExFee>>,
    #[serde(rename = "gasFee", deserialize_with = "Option::deserialize")]
    pub gas_fee: Option<Box<models::SwapPermit2GetPrice200ResponseAnyOfFeesGasFee>>,
}

impl SwapPermit2GetPrice200ResponseAnyOfFees {
    /// Fees to be deducted in this transaction. It contains the `integratorFee`, `zeroExFee` and `gasFee`
    pub fn new(integrator_fee: Option<models::SwapPermit2GetPrice200ResponseAnyOfFeesIntegratorFee>, zero_ex_fee: Option<models::SwapPermit2GetPrice200ResponseAnyOfFeesZeroExFee>, gas_fee: Option<models::SwapPermit2GetPrice200ResponseAnyOfFeesGasFee>) -> SwapPermit2GetPrice200ResponseAnyOfFees {
        SwapPermit2GetPrice200ResponseAnyOfFees {
            integrator_fee: if let Some(x) = integrator_fee {Some(Box::new(x))} else {None},
            zero_ex_fee: if let Some(x) = zero_ex_fee {Some(Box::new(x))} else {None},
            gas_fee: if let Some(x) = gas_fee {Some(Box::new(x))} else {None},
        }
    }
}

