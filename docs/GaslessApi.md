# \GaslessApi

All URIs are relative to *http://api.0x.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gasless_get_price**](GaslessApi.md#gasless_get_price) | **GET** /gasless/price | getPrice
[**gasless_get_quote**](GaslessApi.md#gasless_get_quote) | **GET** /gasless/quote | getQuote
[**gasless_get_status**](GaslessApi.md#gasless_get_status) | **GET** /gasless/status/{tradeHash} | getStatus
[**gasless_submit**](GaslessApi.md#gasless_submit) | **POST** /gasless/submit | submit



## gasless_get_price

> models::GaslessGetPrice200Response gasless_get_price(param_0x_api_key, chain_id, buy_token, sell_token, sell_amount, param_0x_version, taker, swap_fee_recipient, swap_fee_bps, swap_fee_token, trade_surplus_recipient, excluded_sources)
getPrice

Get the indicative price for a gasless swap

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**param_0x_api_key** | **String** | Visit dashboard.0x.org to get your API Key | [required] |
**chain_id** | **i32** | Chain ID. See [here](https://0x.org/docs/next/developer-resources/supported-chains) for the list of supported chains | [required] |
**buy_token** | **String** | The contract address of the token to buy | [required] |
**sell_token** | **String** | The contract address of the token to sell. Native token is not supported | [required] |
**sell_amount** | **String** | The amount of `sellToken` in `sellToken` base units to sell | [required] |
**param_0x_version** | Option<**String**> | The 0x API version to use |  |
**taker** | Option<**String**> | The address which holds the `sellToken` balance and has the allowance set for the swap |  |
**swap_fee_recipient** | Option<**String**> | The wallet address to receive the specified trading fees. You must also specify the `swapFeeToken` and `swapFeeBps` in the request to use this feature. Learn more about setting up a trading fee/commission in the FAQs |  |
**swap_fee_bps** | Option<**i32**> | The amount in Bps of the `swapFeeToken` to charge and deliver to the `swapFeeRecipient`. You must also specify the `swapFeeRecipient` and `swapFeeToken` in the request to use this feature. For security, this field has a default limit of 1000 Bps. If your application requires a higher value, please reach out to us. |  |
**swap_fee_token** | Option<**String**> | The contract address of the token to receive trading fees in. This must be set to the value of either the `buyToken` or the `sellToken`. You must also specify the `swapFeeRecipient` and `swapFeeBps` in the request to use this feature |  |
**trade_surplus_recipient** | Option<**String**> | The address to receive any trade surplus. If specified, this address will receive trade surplus when applicable. Otherwise, the `taker` will receive the surplus |  |
**excluded_sources** | Option<**String**> | Liquidity sources e.g. Uniswap_V3, SushiSwap, 0x_RFQ to exclude from the provided quote. See https://api.0x.org/sources?chainId=<chain_id> with the desired chain's ID for a full list of sources. Separate multiple sources with a comma |  |

### Return type

[**models::GaslessGetPrice200Response**](gasless__getPrice_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gasless_get_quote

> models::GaslessGetQuote200Response gasless_get_quote(param_0x_api_key, chain_id, buy_token, sell_token, sell_amount, taker, param_0x_version, swap_fee_recipient, swap_fee_bps, swap_fee_token, trade_surplus_recipient, excluded_sources)
getQuote

Get the firm quote for a gasless swap

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**param_0x_api_key** | **String** | Visit dashboard.0x.org to get your API Key | [required] |
**chain_id** | **i32** | Chain ID. See [here](https://0x.org/docs/next/developer-resources/supported-chains) for the list of supported chains | [required] |
**buy_token** | **String** | The contract address of the token to buy | [required] |
**sell_token** | **String** | The contract address of the token to sell. Native token is not supported | [required] |
**sell_amount** | **String** | The amount of `sellToken` in `sellToken` base units to sell | [required] |
**taker** | **String** | The address which holds the `sellToken` balance and has the allowance set for the swap | [required] |
**param_0x_version** | Option<**String**> | The 0x API version to use |  |
**swap_fee_recipient** | Option<**String**> | The wallet address to receive the specified trading fees. You must also specify the `swapFeeToken` and `swapFeeBps` in the request to use this feature. Learn more about setting up a trading fee/commission in the FAQs |  |
**swap_fee_bps** | Option<**i32**> | The amount in Bps of the `swapFeeToken` to charge and deliver to the `swapFeeRecipient`. You must also specify the `swapFeeRecipient` and `swapFeeToken` in the request to use this feature. For security, this field has a default limit of 1000 Bps. If your application requires a higher value, please reach out to us. |  |
**swap_fee_token** | Option<**String**> | The contract address of the token to receive trading fees in. This must be set to the value of either the `buyToken` or the `sellToken`. You must also specify the `swapFeeRecipient` and `swapFeeBps` in the request to use this feature |  |
**trade_surplus_recipient** | Option<**String**> | The address to receive any trade surplus. If specified, this address will receive trade surplus when applicable. Otherwise, the `taker` will receive the surplus |  |
**excluded_sources** | Option<**String**> | Liquidity sources e.g. Uniswap_V3, SushiSwap, 0x_RFQ to exclude from the provided quote. See https://api.0x.org/sources?chainId=<chain_id> with the desired chain's ID for a full list of sources. Separate multiple sources with a comma |  |

### Return type

[**models::GaslessGetQuote200Response**](gasless__getQuote_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gasless_get_status

> models::GaslessGetStatus200Response gasless_get_status(param_0x_api_key, chain_id, trade_hash, param_0x_version)
getStatus

Get the status of a gasless swap

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**param_0x_api_key** | **String** | Visit dashboard.0x.org to get your API Key | [required] |
**chain_id** | **i32** | Chain ID. See [here](https://0x.org/docs/next/developer-resources/supported-chains) for the list of supported chains | [required] |
**trade_hash** | **String** | The hash for the trade according to [EIP-712](https://eips.ethereum.org/EIPS/eip-712) | [required] |
**param_0x_version** | Option<**String**> | The 0x API version to use |  |

### Return type

[**models::GaslessGetStatus200Response**](gasless__getStatus_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gasless_submit

> models::GaslessSubmit200Response gasless_submit(param_0x_api_key, gasless_submit_request, param_0x_version)
submit

Submit a gasless swap

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**param_0x_api_key** | **String** | Visit dashboard.0x.org to get your API Key | [required] |
**gasless_submit_request** | [**GaslessSubmitRequest**](GaslessSubmitRequest.md) |  | [required] |
**param_0x_version** | Option<**String**> | The 0x API version to use |  |

### Return type

[**models::GaslessSubmit200Response**](gasless__submit_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

