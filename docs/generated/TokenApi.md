# \TokenApi

All URIs are relative to *https://api.ethereal.trade*

Method | HTTP request | Description
------------- | ------------- | -------------
[**token_controller_get_by_id**](TokenApi.md#token_controller_get_by_id) | **GET** /v1/token/{id} | Returns a token by its id
[**token_controller_initiate_withdraw**](TokenApi.md#token_controller_initiate_withdraw) | **POST** /v1/token/{id}/withdraw | Initiates a withdraw for a specific token in subaccount
[**token_controller_list**](TokenApi.md#token_controller_list) | **GET** /v1/token | Returns a list of all tokens
[**token_controller_list_transfers**](TokenApi.md#token_controller_list_transfers) | **GET** /v1/token/transfer | Returns a list of transfers for the given subaccount
[**token_controller_list_withdraws**](TokenApi.md#token_controller_list_withdraws) | **GET** /v1/token/withdraw | Returns initiated or pending finalize withdraws for the given subaccount



## token_controller_get_by_id

> models::TokenDto token_controller_get_by_id(id)
Returns a token by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::TokenDto**](TokenDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_controller_initiate_withdraw

> models::WithdrawDto token_controller_initiate_withdraw(id, initiate_withdraw_dto)
Initiates a withdraw for a specific token in subaccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**initiate_withdraw_dto** | [**InitiateWithdrawDto**](InitiateWithdrawDto.md) |  | [required] |

### Return type

[**models::WithdrawDto**](WithdrawDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_controller_list

> models::PageOfTokensDtos token_controller_list(order, limit, cursor, deposit_enabled, withdraw_enabled, order_by)
Returns a list of all tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order** | Option<**String**> | Direction to paginate through objects |  |
**limit** | Option<**f64**> | Limit the number of objects to return |  |
**cursor** | Option<**String**> | Pointer to the current object in pagination dataset |  |
**deposit_enabled** | Option<**bool**> | Filters tokens by if its enabled for deposit |  |
**withdraw_enabled** | Option<**bool**> | Filters tokens by if its enabled for withdraw |  |
**order_by** | Option<**String**> | Order by field |  |

### Return type

[**models::PageOfTokensDtos**](PageOfTokensDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_controller_list_transfers

> models::PageOfTransfersDtos token_controller_list_transfers(subaccount_id, order, limit, cursor, statuses, types, order_by, created_after, created_before)
Returns a list of transfers for the given subaccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subaccount_id** | **uuid::Uuid** | Id representing the registered subaccount | [required] |
**order** | Option<**String**> | Direction to paginate through objects |  |
**limit** | Option<**f64**> | Limit the number of objects to return |  |
**cursor** | Option<**String**> | Pointer to the current object in pagination dataset |  |
**statuses** | Option<[**Vec<String>**](String.md)> | Array of transfer statuses to filter by |  |
**types** | Option<[**Vec<String>**](String.md)> | Array of transfer types to filter by |  |
**order_by** | Option<**String**> | Order by field |  |
**created_after** | Option<**f64**> | Filter by transfers created after timestamp exclusive (ms since Unix epoch) |  |
**created_before** | Option<**f64**> | Filter by transfers created before timestamp inclusive (ms since Unix epoch) |  |

### Return type

[**models::PageOfTransfersDtos**](PageOfTransfersDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_controller_list_withdraws

> models::PageOfWithdrawDtos token_controller_list_withdraws(subaccount_id, order, limit, cursor, is_active, order_by)
Returns initiated or pending finalize withdraws for the given subaccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subaccount_id** | **uuid::Uuid** | Id representing the registered subaccount | [required] |
**order** | Option<**String**> | Direction to paginate through objects |  |
**limit** | Option<**f64**> | Limit the number of objects to return |  |
**cursor** | Option<**String**> | Pointer to the current object in pagination dataset |  |
**is_active** | Option<**bool**> | Filters active withdraws |  |
**order_by** | Option<**String**> | Order by field |  |

### Return type

[**models::PageOfWithdrawDtos**](PageOfWithdrawDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

