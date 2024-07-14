# SwapPermit2GetPrice200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_number** | [**serde_json::Value**](serde_json::Value.md) | The block number at which the liquidity sources were sampled to generate the quote. This indicates the freshness of the quote | 
**buy_amount** | [**serde_json::Value**](serde_json::Value.md) | The amount of `buyToken` (in `buyToken` units) that will be bought in the swap | 
**buy_token** | **String** | The contract address of the token to buy in the swap | 
**fees** | [**models::SwapPermit2GetPrice200ResponseAnyOfFees**](swap__permit2__getPrice_200_response_anyOf_fees.md) |  | 
**gas** | Option<[**serde_json::Value**](serde_json::Value.md)> | The estimated gas limit that should be used to send the transaction to guarantee settlement | 
**gas_price** | [**serde_json::Value**](serde_json::Value.md) | The gas price (in wei) that should be used to send the transaction. The transaction needs to be sent with this `gasPrice` for the transaction to be successful | 
**issues** | [**models::SwapPermit2GetPrice200ResponseAnyOfIssues**](swap__permit2__getPrice_200_response_anyOf_issues.md) |  | 
**liquidity_available** | **bool** | This validates the availability of liquidity for the quote requested. No other fields will be returned if it is `false` | 
**min_buy_amount** | [**serde_json::Value**](serde_json::Value.md) | The price which must be met or else the entire transaction will revert. This price is influenced by the `slippageBps` parameter. On-chain sources may encounter price movements from quote to settlement | 
**route** | [**models::SwapPermit2GetPrice200ResponseAnyOfRoute**](swap__permit2__getPrice_200_response_anyOf_route.md) |  | 
**sell_amount** | [**serde_json::Value**](serde_json::Value.md) | The amount of `sellToken` (in `sellToken` units) that will be sold in this swap | 
**sell_token** | **String** | The contract address of the token to sell in the swap | 
**total_network_fee** | Option<[**serde_json::Value**](serde_json::Value.md)> | The estimated total network cost of the swap. On chains where they is no L1 data cost, it is calculated as `gas` * `gasPrice. On chains where there is an L1 data cost, it is calculated as `gas` * `gasPrice + L1 data | 
**zid** | **String** | The unique ZeroEx identifier of the request | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


