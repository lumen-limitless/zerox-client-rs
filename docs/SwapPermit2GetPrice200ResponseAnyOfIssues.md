# SwapPermit2GetPrice200ResponseAnyOfIssues

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowance** | Option<[**models::SwapPermit2GetPrice200ResponseAnyOfIssuesAllowance**](swap__permit2__getPrice_200_response_anyOf_issues_allowance.md)> |  | 
**balance** | Option<[**models::SwapPermit2GetPrice200ResponseAnyOfIssuesBalance**](swap__permit2__getPrice_200_response_anyOf_issues_balance.md)> |  | 
**simulation_incomplete** | **bool** | This is set to `true` when 0x cannot validate the transaction. This happens when the `taker` has an insufficient balance of the `sellToken` and 0x is unable to peform ehanced quote validation with the low balance. Note that this does not necessarily mean that the trade will revert | 
**invalid_sources_passed** | **Vec<String>** | A list of invalid sources present in `excludedSources` request. See https://api.0x.org/sources?chainId= with the desired chain's ID for the list of valid sources | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


