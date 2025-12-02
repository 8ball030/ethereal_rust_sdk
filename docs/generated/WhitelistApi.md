# \WhitelistApi

All URIs are relative to *https://api.ethereal.trade*

Method | HTTP request | Description
------------- | ------------- | -------------
[**whitelist_controller_is_whitelisted**](WhitelistApi.md#whitelist_controller_is_whitelisted) | **GET** /v1/whitelist | Checks if an address is whitelisted



## whitelist_controller_is_whitelisted

> models::WhitelistDto whitelist_controller_is_whitelisted(address)
Checks if an address is whitelisted

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Address of account | [required] |

### Return type

[**models::WhitelistDto**](WhitelistDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

