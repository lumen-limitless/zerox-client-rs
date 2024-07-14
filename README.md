# Rust API client for openapi

These are the API references for the beta version of 0x API v2. If you are looking for API v1 references, [see here](/0x-swap-api/api-references/overview).


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: next
- Package version: next
- Generator version: 7.7.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://api.0x.org*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*GaslessApi* | [**gasless_get_price**](docs/GaslessApi.md#gasless_get_price) | **GET** /gasless/price | getPrice
*GaslessApi* | [**gasless_get_quote**](docs/GaslessApi.md#gasless_get_quote) | **GET** /gasless/quote | getQuote
*GaslessApi* | [**gasless_get_status**](docs/GaslessApi.md#gasless_get_status) | **GET** /gasless/status/{tradeHash} | getStatus
*GaslessApi* | [**gasless_submit**](docs/GaslessApi.md#gasless_submit) | **POST** /gasless/submit | submit
*SourcesApi* | [**sources_get_sources**](docs/SourcesApi.md#sources_get_sources) | **GET** /sources | getSources
*SwapApi* | [**swap_allowance_holder_get_price**](docs/SwapApi.md#swap_allowance_holder_get_price) | **GET** /swap/allowance-holder/price | getPrice (Allowance Holder)
*SwapApi* | [**swap_allowance_holder_get_quote**](docs/SwapApi.md#swap_allowance_holder_get_quote) | **GET** /swap/allowance-holder/quote | getQuote (Allowance Holder)
*SwapApi* | [**swap_permit2_get_price**](docs/SwapApi.md#swap_permit2_get_price) | **GET** /swap/permit2/price | getPrice (Permit2)
*SwapApi* | [**swap_permit2_get_quote**](docs/SwapApi.md#swap_permit2_get_quote) | **GET** /swap/permit2/quote | getQuote (Permit2)


## Documentation For Models

 - [BuyTokenNotAuthorizedForTrade](docs/BuyTokenNotAuthorizedForTrade.md)
 - [BuyTokenNotAuthorizedForTradeData](docs/BuyTokenNotAuthorizedForTradeData.md)
 - [GaslessGetPrice200Response](docs/GaslessGetPrice200Response.md)
 - [GaslessGetPrice200ResponseAnyOf](docs/GaslessGetPrice200ResponseAnyOf.md)
 - [GaslessGetPrice400Response](docs/GaslessGetPrice400Response.md)
 - [GaslessGetQuote200Response](docs/GaslessGetQuote200Response.md)
 - [GaslessGetQuote200ResponseAnyOf](docs/GaslessGetQuote200ResponseAnyOf.md)
 - [GaslessGetQuote200ResponseAnyOfApproval](docs/GaslessGetQuote200ResponseAnyOfApproval.md)
 - [GaslessGetQuote200ResponseAnyOfApprovalEip712](docs/GaslessGetQuote200ResponseAnyOfApprovalEip712.md)
 - [GaslessGetQuote200ResponseAnyOfTrade](docs/GaslessGetQuote200ResponseAnyOfTrade.md)
 - [GaslessGetQuote200ResponseAnyOfTradeEip712](docs/GaslessGetQuote200ResponseAnyOfTradeEip712.md)
 - [GaslessGetQuote400Response](docs/GaslessGetQuote400Response.md)
 - [GaslessGetStatus200Response](docs/GaslessGetStatus200Response.md)
 - [GaslessGetStatus200ResponseAnyOf](docs/GaslessGetStatus200ResponseAnyOf.md)
 - [GaslessGetStatus200ResponseAnyOf1](docs/GaslessGetStatus200ResponseAnyOf1.md)
 - [GaslessGetStatus200ResponseAnyOfApprovalTransactionsInner](docs/GaslessGetStatus200ResponseAnyOfApprovalTransactionsInner.md)
 - [GaslessSubmit200Response](docs/GaslessSubmit200Response.md)
 - [GaslessSubmit400Response](docs/GaslessSubmit400Response.md)
 - [GaslessSubmitRequest](docs/GaslessSubmitRequest.md)
 - [GaslessSubmitRequestApproval](docs/GaslessSubmitRequestApproval.md)
 - [GaslessSubmitRequestApprovalEip712](docs/GaslessSubmitRequestApprovalEip712.md)
 - [GaslessSubmitRequestApprovalEip712AnyOf](docs/GaslessSubmitRequestApprovalEip712AnyOf.md)
 - [GaslessSubmitRequestApprovalEip712AnyOf1](docs/GaslessSubmitRequestApprovalEip712AnyOf1.md)
 - [GaslessSubmitRequestApprovalEip712AnyOf1Message](docs/GaslessSubmitRequestApprovalEip712AnyOf1Message.md)
 - [GaslessSubmitRequestApprovalEip712AnyOf2](docs/GaslessSubmitRequestApprovalEip712AnyOf2.md)
 - [GaslessSubmitRequestApprovalEip712AnyOf2Message](docs/GaslessSubmitRequestApprovalEip712AnyOf2Message.md)
 - [GaslessSubmitRequestApprovalEip712AnyOf2Types](docs/GaslessSubmitRequestApprovalEip712AnyOf2Types.md)
 - [GaslessSubmitRequestApprovalEip712AnyOfMessage](docs/GaslessSubmitRequestApprovalEip712AnyOfMessage.md)
 - [GaslessSubmitRequestApprovalEip712AnyOfTypes](docs/GaslessSubmitRequestApprovalEip712AnyOfTypes.md)
 - [GaslessSubmitRequestApprovalSignature](docs/GaslessSubmitRequestApprovalSignature.md)
 - [GaslessSubmitRequestTrade](docs/GaslessSubmitRequestTrade.md)
 - [GaslessSubmitRequestTradeEip712](docs/GaslessSubmitRequestTradeEip712.md)
 - [GaslessSubmitRequestTradeEip712Message](docs/GaslessSubmitRequestTradeEip712Message.md)
 - [GaslessSubmitRequestTradeEip712MessagePermitted](docs/GaslessSubmitRequestTradeEip712MessagePermitted.md)
 - [GaslessSubmitRequestTradeEip712MessagePermittedAmount](docs/GaslessSubmitRequestTradeEip712MessagePermittedAmount.md)
 - [GaslessSubmitRequestTradeEip712MessageSlippageAndActions](docs/GaslessSubmitRequestTradeEip712MessageSlippageAndActions.md)
 - [InputInvalid](docs/InputInvalid.md)
 - [InputInvalidData](docs/InputInvalidData.md)
 - [InputInvalidDataDetailsInner](docs/InputInvalidDataDetailsInner.md)
 - [InsufficientBalance](docs/InsufficientBalance.md)
 - [InsufficientBalanceData](docs/InsufficientBalanceData.md)
 - [InsufficientBalanceOrAllowance](docs/InsufficientBalanceOrAllowance.md)
 - [InsufficientBalanceOrAllowanceData](docs/InsufficientBalanceOrAllowanceData.md)
 - [InternalServerError](docs/InternalServerError.md)
 - [InvalidSignature](docs/InvalidSignature.md)
 - [InvalidSignatureData](docs/InvalidSignatureData.md)
 - [InvalidSigner](docs/InvalidSigner.md)
 - [InvalidSignerData](docs/InvalidSignerData.md)
 - [MetaTransactionExpiryTooSoon](docs/MetaTransactionExpiryTooSoon.md)
 - [MetaTransactionExpiryTooSoonData](docs/MetaTransactionExpiryTooSoonData.md)
 - [MetaTransactionInvalid](docs/MetaTransactionInvalid.md)
 - [MetaTransactionInvalidData](docs/MetaTransactionInvalidData.md)
 - [MetaTransactionStatusNotFound](docs/MetaTransactionStatusNotFound.md)
 - [PendingTradesAlreadyExist](docs/PendingTradesAlreadyExist.md)
 - [PendingTradesAlreadyExistData](docs/PendingTradesAlreadyExistData.md)
 - [SellAmountTooSmall](docs/SellAmountTooSmall.md)
 - [SellAmountTooSmallData](docs/SellAmountTooSmallData.md)
 - [SellTokenNotAuthorizedForTrade](docs/SellTokenNotAuthorizedForTrade.md)
 - [SourcesGetSources200Response](docs/SourcesGetSources200Response.md)
 - [SwapAllowanceHolderGetQuote200Response](docs/SwapAllowanceHolderGetQuote200Response.md)
 - [SwapAllowanceHolderGetQuote200ResponseAnyOf](docs/SwapAllowanceHolderGetQuote200ResponseAnyOf.md)
 - [SwapPermit2GetPrice200Response](docs/SwapPermit2GetPrice200Response.md)
 - [SwapPermit2GetPrice200ResponseAnyOf](docs/SwapPermit2GetPrice200ResponseAnyOf.md)
 - [SwapPermit2GetPrice200ResponseAnyOf1](docs/SwapPermit2GetPrice200ResponseAnyOf1.md)
 - [SwapPermit2GetPrice200ResponseAnyOfFees](docs/SwapPermit2GetPrice200ResponseAnyOfFees.md)
 - [SwapPermit2GetPrice200ResponseAnyOfFeesGasFee](docs/SwapPermit2GetPrice200ResponseAnyOfFeesGasFee.md)
 - [SwapPermit2GetPrice200ResponseAnyOfFeesIntegratorFee](docs/SwapPermit2GetPrice200ResponseAnyOfFeesIntegratorFee.md)
 - [SwapPermit2GetPrice200ResponseAnyOfFeesZeroExFee](docs/SwapPermit2GetPrice200ResponseAnyOfFeesZeroExFee.md)
 - [SwapPermit2GetPrice200ResponseAnyOfIssues](docs/SwapPermit2GetPrice200ResponseAnyOfIssues.md)
 - [SwapPermit2GetPrice200ResponseAnyOfIssuesAllowance](docs/SwapPermit2GetPrice200ResponseAnyOfIssuesAllowance.md)
 - [SwapPermit2GetPrice200ResponseAnyOfIssuesBalance](docs/SwapPermit2GetPrice200ResponseAnyOfIssuesBalance.md)
 - [SwapPermit2GetPrice200ResponseAnyOfRoute](docs/SwapPermit2GetPrice200ResponseAnyOfRoute.md)
 - [SwapPermit2GetPrice200ResponseAnyOfRouteFillsInner](docs/SwapPermit2GetPrice200ResponseAnyOfRouteFillsInner.md)
 - [SwapPermit2GetPrice200ResponseAnyOfRouteTokensInner](docs/SwapPermit2GetPrice200ResponseAnyOfRouteTokensInner.md)
 - [SwapPermit2GetPrice400Response](docs/SwapPermit2GetPrice400Response.md)
 - [SwapPermit2GetPrice422Response](docs/SwapPermit2GetPrice422Response.md)
 - [SwapPermit2GetPrice500Response](docs/SwapPermit2GetPrice500Response.md)
 - [SwapPermit2GetQuote200Response](docs/SwapPermit2GetQuote200Response.md)
 - [SwapPermit2GetQuote200ResponseAnyOf](docs/SwapPermit2GetQuote200ResponseAnyOf.md)
 - [SwapPermit2GetQuote200ResponseAnyOfFees](docs/SwapPermit2GetQuote200ResponseAnyOfFees.md)
 - [SwapPermit2GetQuote200ResponseAnyOfPermit2](docs/SwapPermit2GetQuote200ResponseAnyOfPermit2.md)
 - [SwapPermit2GetQuote200ResponseAnyOfPermit2Eip712](docs/SwapPermit2GetQuote200ResponseAnyOfPermit2Eip712.md)
 - [SwapPermit2GetQuote200ResponseAnyOfPermit2Eip712Domain](docs/SwapPermit2GetQuote200ResponseAnyOfPermit2Eip712Domain.md)
 - [SwapPermit2GetQuote200ResponseAnyOfPermit2Eip712MessageValue](docs/SwapPermit2GetQuote200ResponseAnyOfPermit2Eip712MessageValue.md)
 - [SwapPermit2GetQuote200ResponseAnyOfPermit2Eip712TypesValueInner](docs/SwapPermit2GetQuote200ResponseAnyOfPermit2Eip712TypesValueInner.md)
 - [SwapPermit2GetQuote200ResponseAnyOfTransaction](docs/SwapPermit2GetQuote200ResponseAnyOfTransaction.md)
 - [TokenNotSupported](docs/TokenNotSupported.md)
 - [Uncategorized](docs/Uncategorized.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



