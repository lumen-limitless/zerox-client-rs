# GaslessGetStatus200ResponseAnyOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**approval_transactions** | Option<[**Vec<models::GaslessGetStatus200ResponseAnyOfApprovalTransactionsInner>**](gasless__getStatus_200_response_anyOf_approvalTransactions_inner.md)> | Details of the gasless approval transaction | [optional]
**status** | **String** | `pending` means that the order has been queued on 0x. `submitted` means that it has been submitted onchain,`succeeded` means it has been included in a block and `confirmed` means it has at least 3 confirmations onchain | 
**transactions** | [**Vec<models::GaslessGetStatus200ResponseAnyOfApprovalTransactionsInner>**](gasless__getStatus_200_response_anyOf_approvalTransactions_inner.md) | Details of the gasless swap transaction. If the trade is `pending`, no transaction will be returned. If `submitted`, multiple transactions may be returned, but only one will be mined. If `successful` and `confirmed`, the mined transaction will be returned | 
**zid** | **String** | The unique ZeroEx identifier of the request | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


