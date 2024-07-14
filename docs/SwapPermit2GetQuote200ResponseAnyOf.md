# SwapPermit2GetQuote200ResponseAnyOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_number** | [**serde_json::Value**](serde_json::Value.md) | The block number at which the liquidity sources were sampled to generate the quote. This indicates the freshness of the quote | 
**buy_amount** | [**serde_json::Value**](serde_json::Value.md) | The amount of `buyToken` (in `buyToken` units) that will be bought in the swap | 
**buy_token** | **String** | The contract address of the token to buy in the swap | 
**fees** | [**models::SwapPermit2GetQuote200ResponseAnyOfFees**](swap__permit2__getQuote_200_response_anyOf_fees.md) |  | 
**issues** | [**models::SwapPermit2GetPrice200ResponseAnyOfIssues**](swap__permit2__getPrice_200_response_anyOf_issues.md) |  | 
**liquidity_available** | **bool** | This validates the availability of liquidity for the quote requested. The rest of the fields will only be returned if `true` | 
**min_buy_amount** | [**serde_json::Value**](serde_json::Value.md) | The price which must be met or else the transaction will revert. This price is influenced by the `slippageBps` parameter. On-chain sources may encounter price movements from quote to settlement | 
**permit2** | Option<[**models::SwapPermit2GetQuote200ResponseAnyOfPermit2**](swap__permit2__getQuote_200_response_anyOf_permit2.md)> |  | 
**route** | [**models::SwapPermit2GetPrice200ResponseAnyOfRoute**](swap__permit2__getPrice_200_response_anyOf_route.md) |  | 
**sell_amount** | [**serde_json::Value**](serde_json::Value.md) | The amount of `sellToken` (in `sellToken` units) that will be sold in this swap | 
**sell_token** | **String** | The contract address of the token to sell in the swap | 
**total_network_fee** | Option<[**serde_json::Value**](serde_json::Value.md)> | The estimated total network cost of the swap. On chains where they is no L1 data cost, it is calculated as `gas` * `gasPrice. On chains where there is an L1 data cost, it is calculated as `gas` * `gasPrice + L1 data | 
**transaction** | [**models::SwapPermit2GetQuote200ResponseAnyOfTransaction**](swap__permit2__getQuote_200_response_anyOf_transaction.md) |  | 
**zid** | **String** | The unique ZeroEx identifier of the request | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


