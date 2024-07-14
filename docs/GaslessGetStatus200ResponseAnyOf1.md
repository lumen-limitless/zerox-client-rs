# GaslessGetStatus200ResponseAnyOf1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**approval_transactions** | Option<[**Vec<models::GaslessGetStatus200ResponseAnyOfApprovalTransactionsInner>**](gasless__getStatus_200_response_anyOf_approvalTransactions_inner.md)> | Details of the gasless approval transaction | [optional]
**reason** | **String** | This provides more context about why the transaction failed | 
**status** | **String** | `failed` means that the order failed to be submitted onchain | 
**transactions** | [**Vec<models::GaslessGetStatus200ResponseAnyOfApprovalTransactionsInner>**](gasless__getStatus_200_response_anyOf_approvalTransactions_inner.md) | Details of the gasless swap transaction. If the trade status is `failed`, there may be 0 (if it failed before submission) to multiple transactions (if the transaction reverted) | 
**zid** | **String** | The unique ZeroEx identifier of the request | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


