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
pub struct MetaTransactionInvalidData {
    /// The unique ZeroEx identifier of the request
    #[serde(rename = "zid")]
    pub zid: String,
    /// The hash of the meta-transaction provided by the caller
    #[serde(rename = "metaTransactionHash")]
    pub meta_transaction_hash: String,
}

impl MetaTransactionInvalidData {
    pub fn new(zid: String, meta_transaction_hash: String) -> MetaTransactionInvalidData {
        MetaTransactionInvalidData {
            zid,
            meta_transaction_hash,
        }
    }
}

