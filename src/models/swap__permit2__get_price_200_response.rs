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
pub struct SwapPermit2GetPrice200Response {
    /// The block number at which the liquidity sources were sampled to generate the quote. This indicates the freshness of the quote
    #[serde(rename = "blockNumber")]
    pub block_number: Box<serde_json::Value>,
    /// The amount of `buyToken` (in `buyToken` units) that will be bought in the swap
    #[serde(rename = "buyAmount")]
    pub buy_amount: Box<serde_json::Value>,
    /// The contract address of the token to buy in the swap
    #[serde(rename = "buyToken")]
    pub buy_token: String,
    #[serde(rename = "fees")]
    pub fees: Box<models::SwapPermit2GetPrice200ResponseAnyOfFees>,
    /// The estimated gas limit that should be used to send the transaction to guarantee settlement
    #[serde(rename = "gas", deserialize_with = "Option::deserialize")]
    pub gas: Option<Box<serde_json::Value>>,
    /// The gas price (in wei) that should be used to send the transaction. The transaction needs to be sent with this `gasPrice` for the transaction to be successful
    #[serde(rename = "gasPrice")]
    pub gas_price: Box<serde_json::Value>,
    #[serde(rename = "issues")]
    pub issues: Box<models::SwapPermit2GetPrice200ResponseAnyOfIssues>,
    /// This validates the availability of liquidity for the quote requested. No other fields will be returned if it is `false`
    #[serde(rename = "liquidityAvailable")]
    pub liquidity_available: bool,
    /// The price which must be met or else the entire transaction will revert. This price is influenced by the `slippageBps` parameter. On-chain sources may encounter price movements from quote to settlement
    #[serde(rename = "minBuyAmount")]
    pub min_buy_amount: Box<serde_json::Value>,
    #[serde(rename = "route")]
    pub route: Box<models::SwapPermit2GetPrice200ResponseAnyOfRoute>,
    /// The amount of `sellToken` (in `sellToken` units) that will be sold in this swap
    #[serde(rename = "sellAmount")]
    pub sell_amount: Box<serde_json::Value>,
    /// The contract address of the token to sell in the swap
    #[serde(rename = "sellToken")]
    pub sell_token: String,
    /// The estimated total network cost of the swap. On chains where they is no L1 data cost, it is calculated as `gas` * `gasPrice. On chains where there is an L1 data cost, it is calculated as `gas` * `gasPrice + L1 data
    #[serde(rename = "totalNetworkFee", deserialize_with = "Option::deserialize")]
    pub total_network_fee: Option<Box<serde_json::Value>>,
    /// The unique ZeroEx identifier of the request
    #[serde(rename = "zid")]
    pub zid: String,
}

impl SwapPermit2GetPrice200Response {
    pub fn new(block_number: serde_json::Value, buy_amount: serde_json::Value, buy_token: String, fees: models::SwapPermit2GetPrice200ResponseAnyOfFees, gas: Option<serde_json::Value>, gas_price: serde_json::Value, issues: models::SwapPermit2GetPrice200ResponseAnyOfIssues, liquidity_available: bool, min_buy_amount: serde_json::Value, route: models::SwapPermit2GetPrice200ResponseAnyOfRoute, sell_amount: serde_json::Value, sell_token: String, total_network_fee: Option<serde_json::Value>, zid: String) -> SwapPermit2GetPrice200Response {
        SwapPermit2GetPrice200Response {
            block_number: Box::new(block_number),
            buy_amount: Box::new(buy_amount),
            buy_token,
            fees: Box::new(fees),
            gas: if let Some(x) = gas {Some(Box::new(x))} else {None},
            gas_price: Box::new(gas_price),
            issues: Box::new(issues),
            liquidity_available,
            min_buy_amount: Box::new(min_buy_amount),
            route: Box::new(route),
            sell_amount: Box::new(sell_amount),
            sell_token,
            total_network_fee: if let Some(x) = total_network_fee {Some(Box::new(x))} else {None},
            zid,
        }
    }
}

