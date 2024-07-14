# InsufficientBalanceOrAllowanceData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**zid** | **String** | The unique ZeroEx identifier of the request | 
**meta_transaction_hash** | **String** | The hash of the meta-transaction provided by the caller | 
**taker** | **String** | The intended signer of the meta-transaction | 
**sell_token** | **String** | The sell token | 
**sell_amount** | [**serde_json::Value**](serde_json::Value.md) | The sell amount | 
**min_balance_or_allowance** | [**serde_json::Value**](serde_json::Value.md) | The smaller value of the balance or the allowance of the taker | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


