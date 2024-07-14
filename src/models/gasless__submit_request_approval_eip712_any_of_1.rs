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
pub struct GaslessSubmitRequestApprovalEip712AnyOf1 {
    #[serde(rename = "types")]
    pub types: Box<models::GaslessSubmitRequestApprovalEip712AnyOfTypes>,
    #[serde(rename = "domain")]
    pub domain: Box<models::SwapPermit2GetQuote200ResponseAnyOfPermit2Eip712Domain>,
    #[serde(rename = "message")]
    pub message: Box<models::GaslessSubmitRequestApprovalEip712AnyOf1Message>,
    #[serde(rename = "primaryType")]
    pub primary_type: PrimaryType,
}

impl GaslessSubmitRequestApprovalEip712AnyOf1 {
    pub fn new(types: models::GaslessSubmitRequestApprovalEip712AnyOfTypes, domain: models::SwapPermit2GetQuote200ResponseAnyOfPermit2Eip712Domain, message: models::GaslessSubmitRequestApprovalEip712AnyOf1Message, primary_type: PrimaryType) -> GaslessSubmitRequestApprovalEip712AnyOf1 {
        GaslessSubmitRequestApprovalEip712AnyOf1 {
            types: Box::new(types),
            domain: Box::new(domain),
            message: Box::new(message),
            primary_type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PrimaryType {
    #[serde(rename = "Permit")]
    Permit,
}

impl Default for PrimaryType {
    fn default() -> PrimaryType {
        Self::Permit
    }
}

