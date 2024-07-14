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

/// GaslessGetQuote200ResponseAnyOfTrade : This is the “trade” object which contains the necessary information to process a gasless trade
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GaslessGetQuote200ResponseAnyOfTrade {
    /// The transaction type determined by the trade route. This is currently just `settler_metatransaction` and could expand to more types in the future
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The hash for the trade according to [EIP-712](https://eips.ethereum.org/EIPS/eip-712). If you compute the hash from eip712 field, it should match the value of this field
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "eip712")]
    pub eip712: Box<models::GaslessGetQuote200ResponseAnyOfTradeEip712>,
}

impl GaslessGetQuote200ResponseAnyOfTrade {
    /// This is the “trade” object which contains the necessary information to process a gasless trade
    pub fn new(r#type: Type, hash: String, eip712: models::GaslessGetQuote200ResponseAnyOfTradeEip712) -> GaslessGetQuote200ResponseAnyOfTrade {
        GaslessGetQuote200ResponseAnyOfTrade {
            r#type,
            hash,
            eip712: Box::new(eip712),
        }
    }
}
/// The transaction type determined by the trade route. This is currently just `settler_metatransaction` and could expand to more types in the future
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "settler_metatransaction")]
    SettlerMetatransaction,
}

impl Default for Type {
    fn default() -> Type {
        Self::SettlerMetatransaction
    }
}

