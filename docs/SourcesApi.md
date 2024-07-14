# \SourcesApi

All URIs are relative to *http://api.0x.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sources_get_sources**](SourcesApi.md#sources_get_sources) | **GET** /sources | getSources



## sources_get_sources

> models::SourcesGetSources200Response sources_get_sources(param_0x_api_key, chain_id, param_0x_version)
getSources

Get the list of supported sources

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**param_0x_api_key** | **String** | Visit dashboard.0x.org to get your API Key | [required] |
**chain_id** | **i32** | Chain ID. See [here](https://0x.org/docs/next/developer-resources/supported-chains) for the list of supported chains | [required] |
**param_0x_version** | Option<**String**> | The 0x API version to use |  |

### Return type

[**models::SourcesGetSources200Response**](sources__getSources_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

