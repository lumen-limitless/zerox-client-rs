# SwapPermit2GetQuote200ResponseAnyOfTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**to** | **String** | The address of the target contract to send call `data` to | 
**data** | **String** | The calldata containing transaction execution details to be sent to the `to` address | 
**gas** | Option<[**serde_json::Value**](serde_json::Value.md)> | The estimated gas limit that should be used to send the transaction to guarantee settlement | 
**gas_price** | [**serde_json::Value**](serde_json::Value.md) | The gas price (in wei) that should be used to send the transaction | 
**value** | [**serde_json::Value**](serde_json::Value.md) | The amount of ether (in wei) that should be sent with the transaction | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


