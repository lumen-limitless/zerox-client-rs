# \SwapApi

All URIs are relative to *http://api.0x.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**swap_allowance_holder_get_price**](SwapApi.md#swap_allowance_holder_get_price) | **GET** /swap/allowance-holder/price | getPrice (Allowance Holder)
[**swap_allowance_holder_get_quote**](SwapApi.md#swap_allowance_holder_get_quote) | **GET** /swap/allowance-holder/quote | getQuote (Allowance Holder)
[**swap_permit2_get_price**](SwapApi.md#swap_permit2_get_price) | **GET** /swap/permit2/price | getPrice (Permit2)
[**swap_permit2_get_quote**](SwapApi.md#swap_permit2_get_quote) | **GET** /swap/permit2/quote | getQuote (Permit2)



## swap_allowance_holder_get_price

> models::SwapPermit2GetPrice200Response swap_allowance_holder_get_price(param_0x_api_key, chain_id, buy_token, sell_token, sell_amount, param_0x_version, taker, tx_origin, swap_fee_recipient, swap_fee_bps, swap_fee_token, trade_surplus_recipient, gas_price, slippage_bps, excluded_sources)
getPrice (Allowance Holder)

Get the indicative price for a swap using Allowance Holder to set allowances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**param_0x_api_key** | **String** | Visit dashboard.0x.org to get your API Key | [required] |
**chain_id** | **i32** | Chain ID. See [here](https://0x.org/docs/next/developer-resources/supported-chains) for the list of supported chains | [required] |
**buy_token** | **String** | The contract address of the token to buy | [required] |
**sell_token** | **String** | The contract address of the token to sell | [required] |
**sell_amount** | **String** | The amount of `sellToken` in `sellToken` base units to sell | [required] |
**param_0x_version** | Option<**String**> | The 0x API version to use |  |
**taker** | Option<**String**> | The address which holds the `sellToken` balance and has the allowance set for the swap |  |
**tx_origin** | Option<**String**> |  |  |
**swap_fee_recipient** | Option<**String**> | The wallet address to receive the specified trading fees. You must also specify the `swapFeeToken` and `swapFeeBps` in the request to use this feature. Learn more about setting up a trading fee/commission in the FAQs |  |
**swap_fee_bps** | Option<**i32**> | The amount in Bps of the `swapFeeToken` to charge and deliver to the `swapFeeRecipient`. You must also specify the `swapFeeRecipient` and `swapFeeToken` in the request to use this feature. For security, this field has a default limit of 1000 Bps. If your application requires a higher value, please reach out to us. |  |
**swap_fee_token** | Option<**String**> | The contract address of the token to receive trading fees in. This must be set to the value of either the `buyToken` or the `sellToken`. You must also specify the `swapFeeRecipient` and `swapFeeBps` in the request to use this feature |  |
**trade_surplus_recipient** | Option<**String**> | The address to receive any trade surplus. If specified, this address will receive trade surplus when applicable. Otherwise, the `taker` will receive the surplus |  |
**gas_price** | Option<**String**> | The target gas price (in wei) for the swap transaction. If not provided, the default value is based on the 0x gas price oracle |  |
**slippage_bps** | Option<**i32**> | The maximum acceptable slippage of the `buyToken` in Bps. If this parameter is set to 0, no slippage will be tolerated. If not provided, the default slippage tolerance is 100Bps |  |[default to 100]
**excluded_sources** | Option<**String**> | Liquidity sources e.g. Uniswap_V3, SushiSwap, 0x_RFQ to exclude from the provided quote. See https://api.0x.org/sources?chainId=<chain_id> with the desired chain's ID for a full list of sources. Separate multiple sources with a comma |  |

### Return type

[**models::SwapPermit2GetPrice200Response**](swap__permit2__getPrice_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swap_allowance_holder_get_quote

> models::SwapAllowanceHolderGetQuote200Response swap_allowance_holder_get_quote(param_0x_api_key, chain_id, buy_token, sell_token, sell_amount, taker, param_0x_version, tx_origin, swap_fee_recipient, swap_fee_bps, swap_fee_token, trade_surplus_recipient, gas_price, slippage_bps, excluded_sources)
getQuote (Allowance Holder)

Get the firm quote for a swap using Allowance Holder to set allowances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**param_0x_api_key** | **String** | Visit dashboard.0x.org to get your API Key | [required] |
**chain_id** | **i32** | Chain ID. See [here](https://0x.org/docs/next/developer-resources/supported-chains) for the list of supported chains | [required] |
**buy_token** | **String** | The contract address of the token to buy | [required] |
**sell_token** | **String** | The contract address of the token to sell | [required] |
**sell_amount** | **String** | The amount of `sellToken` in `sellToken` base units to sell | [required] |
**taker** | **String** | The address which holds the `sellToken` balance and has the allowance set for the swap | [required] |
**param_0x_version** | Option<**String**> | The 0x API version to use |  |
**tx_origin** | Option<**String**> |  |  |
**swap_fee_recipient** | Option<**String**> | The wallet address to receive the specified trading fees. You must also specify the `swapFeeToken` and `swapFeeBps` in the request to use this feature. Learn more about setting up a trading fee/commission in the FAQs |  |
**swap_fee_bps** | Option<**i32**> | The amount in Bps of the `swapFeeToken` to charge and deliver to the `swapFeeRecipient`. You must also specify the `swapFeeRecipient` and `swapFeeToken` in the request to use this feature. For security, this field has a default limit of 1000 Bps. If your application requires a higher value, please reach out to us. |  |
**swap_fee_token** | Option<**String**> | The contract address of the token to receive trading fees in. This must be set to the value of either the `buyToken` or the `sellToken`. You must also specify the `swapFeeRecipient` and `swapFeeBps` in the request to use this feature |  |
**trade_surplus_recipient** | Option<**String**> | The address to receive any trade surplus. If specified, this address will receive trade surplus when applicable. Otherwise, the `taker` will receive the surplus |  |
**gas_price** | Option<**String**> | The target gas price (in wei) for the swap transaction. If not provided, the default value is based on the 0x gas price oracle |  |
**slippage_bps** | Option<**i32**> | The maximum acceptable slippage of the `buyToken` in Bps. If this parameter is set to 0, no slippage will be tolerated. If not provided, the default slippage tolerance is 100Bps |  |[default to 100]
**excluded_sources** | Option<**String**> | Liquidity sources e.g. Uniswap_V3, SushiSwap, 0x_RFQ to exclude from the provided quote. See https://api.0x.org/sources?chainId=<chain_id> with the desired chain's ID for a full list of sources. Separate multiple sources with a comma |  |

### Return type

[**models::SwapAllowanceHolderGetQuote200Response**](swap__allowanceHolder__getQuote_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swap_permit2_get_price

> models::SwapPermit2GetPrice200Response swap_permit2_get_price(param_0x_api_key, chain_id, buy_token, sell_token, sell_amount, param_0x_version, taker, tx_origin, swap_fee_recipient, swap_fee_bps, swap_fee_token, trade_surplus_recipient, gas_price, slippage_bps, excluded_sources)
getPrice (Permit2)

Get the indicative price for a swap using Permit2 to set allowances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**param_0x_api_key** | **String** | Visit dashboard.0x.org to get your API Key | [required] |
**chain_id** | **i32** | Chain ID. See [here](https://0x.org/docs/next/developer-resources/supported-chains) for the list of supported chains | [required] |
**buy_token** | **String** | The contract address of the token to buy | [required] |
**sell_token** | **String** | The contract address of the token to sell | [required] |
**sell_amount** | **String** | The amount of `sellToken` in `sellToken` base units to sell | [required] |
**param_0x_version** | Option<**String**> | The 0x API version to use |  |
**taker** | Option<**String**> | The address which holds the `sellToken` balance and has the allowance set for the swap |  |
**tx_origin** | Option<**String**> |  |  |
**swap_fee_recipient** | Option<**String**> | The wallet address to receive the specified trading fees. You must also specify the `swapFeeToken` and `swapFeeBps` in the request to use this feature. Learn more about setting up a trading fee/commission in the FAQs |  |
**swap_fee_bps** | Option<**i32**> | The amount in Bps of the `swapFeeToken` to charge and deliver to the `swapFeeRecipient`. You must also specify the `swapFeeRecipient` and `swapFeeToken` in the request to use this feature. For security, this field has a default limit of 1000 Bps. If your application requires a higher value, please reach out to us. |  |
**swap_fee_token** | Option<**String**> | The contract address of the token to receive trading fees in. This must be set to the value of either the `buyToken` or the `sellToken`. You must also specify the `swapFeeRecipient` and `swapFeeBps` in the request to use this feature |  |
**trade_surplus_recipient** | Option<**String**> | The address to receive any trade surplus. If specified, this address will receive trade surplus when applicable. Otherwise, the `taker` will receive the surplus |  |
**gas_price** | Option<**String**> | The target gas price (in wei) for the swap transaction. If not provided, the default value is based on the 0x gas price oracle |  |
**slippage_bps** | Option<**i32**> | The maximum acceptable slippage of the `buyToken` in Bps. If this parameter is set to 0, no slippage will be tolerated. If not provided, the default slippage tolerance is 100Bps |  |[default to 100]
**excluded_sources** | Option<**String**> | Liquidity sources e.g. Uniswap_V3, SushiSwap, 0x_RFQ to exclude from the provided quote. See https://api.0x.org/sources?chainId=<chain_id> with the desired chain's ID for a full list of sources. Separate multiple sources with a comma |  |

### Return type

[**models::SwapPermit2GetPrice200Response**](swap__permit2__getPrice_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swap_permit2_get_quote

> models::SwapPermit2GetQuote200Response swap_permit2_get_quote(param_0x_api_key, chain_id, buy_token, sell_token, sell_amount, taker, param_0x_version, tx_origin, swap_fee_recipient, swap_fee_bps, swap_fee_token, trade_surplus_recipient, gas_price, slippage_bps, excluded_sources)
getQuote (Permit2)

Get the firm quote for a swap using Permit2 to set allowances

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**param_0x_api_key** | **String** | Visit dashboard.0x.org to get your API Key | [required] |
**chain_id** | **i32** | Chain ID. See [here](https://0x.org/docs/next/developer-resources/supported-chains) for the list of supported chains | [required] |
**buy_token** | **String** | The contract address of the token to buy | [required] |
**sell_token** | **String** | The contract address of the token to sell | [required] |
**sell_amount** | **String** | The amount of `sellToken` in `sellToken` base units to sell | [required] |
**taker** | **String** | The address which holds the `sellToken` balance and has the allowance set for the swap | [required] |
**param_0x_version** | Option<**String**> | The 0x API version to use |  |
**tx_origin** | Option<**String**> |  |  |
**swap_fee_recipient** | Option<**String**> | The wallet address to receive the specified trading fees. You must also specify the `swapFeeToken` and `swapFeeBps` in the request to use this feature. Learn more about setting up a trading fee/commission in the FAQs |  |
**swap_fee_bps** | Option<**i32**> | The amount in Bps of the `swapFeeToken` to charge and deliver to the `swapFeeRecipient`. You must also specify the `swapFeeRecipient` and `swapFeeToken` in the request to use this feature. For security, this field has a default limit of 1000 Bps. If your application requires a higher value, please reach out to us. |  |
**swap_fee_token** | Option<**String**> | The contract address of the token to receive trading fees in. This must be set to the value of either the `buyToken` or the `sellToken`. You must also specify the `swapFeeRecipient` and `swapFeeBps` in the request to use this feature |  |
**trade_surplus_recipient** | Option<**String**> | The address to receive any trade surplus. If specified, this address will receive trade surplus when applicable. Otherwise, the `taker` will receive the surplus |  |
**gas_price** | Option<**String**> | The target gas price (in wei) for the swap transaction. If not provided, the default value is based on the 0x gas price oracle |  |
**slippage_bps** | Option<**i32**> | The maximum acceptable slippage of the `buyToken` in Bps. If this parameter is set to 0, no slippage will be tolerated. If not provided, the default slippage tolerance is 100Bps |  |[default to 100]
**excluded_sources** | Option<**String**> | Liquidity sources e.g. Uniswap_V3, SushiSwap, 0x_RFQ to exclude from the provided quote. See https://api.0x.org/sources?chainId=<chain_id> with the desired chain's ID for a full list of sources. Separate multiple sources with a comma |  |

### Return type

[**models::SwapPermit2GetQuote200Response**](swap__permit2__getQuote_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

