# \SubaccountApi

All URIs are relative to *https://api.ethereal.trade*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subaccount_controller_get_by_subaccount_id**](SubaccountApi.md#subaccount_controller_get_by_subaccount_id) | **GET** /v1/subaccount/{id} | Returns subaccount by id
[**subaccount_controller_list_by_account**](SubaccountApi.md#subaccount_controller_list_by_account) | **GET** /v1/subaccount | Returns subaccounts for the given account
[**subaccount_controller_list_subaccount_balances**](SubaccountApi.md#subaccount_controller_list_subaccount_balances) | **GET** /v1/subaccount/balance | Returns subaccount balances for given subaccount



## subaccount_controller_get_by_subaccount_id

> models::SubaccountDto subaccount_controller_get_by_subaccount_id(id)
Returns subaccount by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::SubaccountDto**](SubaccountDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subaccount_controller_list_by_account

> models::PageOfSubaccountDtos subaccount_controller_list_by_account(sender, order, limit, cursor, name, order_by)
Returns subaccounts for the given account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sender** | **String** | Address of the sender | [required] |
**order** | Option<**String**> | Direction to paginate through objects |  |
**limit** | Option<**f64**> | Limit the number of objects to return |  |
**cursor** | Option<**String**> | Pointer to the current object in pagination dataset |  |
**name** | Option<**String**> | Bytes32 encoded subaccount name (0x prefix, zero padded) |  |
**order_by** | Option<**String**> | Order by field |  |

### Return type

[**models::PageOfSubaccountDtos**](PageOfSubaccountDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subaccount_controller_list_subaccount_balances

> models::PageOfSubaccountBalanceDtos subaccount_controller_list_subaccount_balances(subaccount_id, order, limit, cursor, order_by)
Returns subaccount balances for given subaccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subaccount_id** | **uuid::Uuid** | Id representing the registered subaccount | [required] |
**order** | Option<**String**> | Direction to paginate through objects |  |
**limit** | Option<**f64**> | Limit the number of objects to return |  |
**cursor** | Option<**String**> | Pointer to the current object in pagination dataset |  |
**order_by** | Option<**String**> | Order by field |  |

### Return type

[**models::PageOfSubaccountBalanceDtos**](PageOfSubaccountBalanceDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

