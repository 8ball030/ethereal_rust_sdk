# \LinkedSignerApi

All URIs are relative to *https://api.ethereal.trade*

Method | HTTP request | Description
------------- | ------------- | -------------
[**linked_signer_controller_get_account_quota**](LinkedSignerApi.md#linked_signer_controller_get_account_quota) | **GET** /v1/linked-signer/quota | Returns the current signer config for a subaccount
[**linked_signer_controller_get_signer**](LinkedSignerApi.md#linked_signer_controller_get_signer) | **GET** /v1/linked-signer/{id} | Returns a signer by its id
[**linked_signer_controller_link_signer**](LinkedSignerApi.md#linked_signer_controller_link_signer) | **POST** /v1/linked-signer/link | Links a signer address with the sender address for order delegation
[**linked_signer_controller_list_by_subaccount_id**](LinkedSignerApi.md#linked_signer_controller_list_by_subaccount_id) | **GET** /v1/linked-signer | List signers for a subaccount
[**linked_signer_controller_refresh_signer**](LinkedSignerApi.md#linked_signer_controller_refresh_signer) | **POST** /v1/linked-signer/refresh | Refreshes the expiry of a linked signer
[**linked_signer_controller_revoke_signer**](LinkedSignerApi.md#linked_signer_controller_revoke_signer) | **DELETE** /v1/linked-signer/revoke | Revokes a signer address from a subaccount



## linked_signer_controller_get_account_quota

> models::AccountSignerQuotaDto linked_signer_controller_get_account_quota(subaccount_id)
Returns the current signer config for a subaccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subaccount_id** | **uuid::Uuid** | Id representing the registered subaccount | [required] |

### Return type

[**models::AccountSignerQuotaDto**](AccountSignerQuotaDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## linked_signer_controller_get_signer

> models::SignerDto linked_signer_controller_get_signer(id)
Returns a signer by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::SignerDto**](SignerDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## linked_signer_controller_link_signer

> models::SignerDto linked_signer_controller_link_signer(link_signer_dto)
Links a signer address with the sender address for order delegation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_signer_dto** | [**LinkSignerDto**](LinkSignerDto.md) |  | [required] |

### Return type

[**models::SignerDto**](SignerDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## linked_signer_controller_list_by_subaccount_id

> models::PageOfSignersDto linked_signer_controller_list_by_subaccount_id(subaccount_id, order, limit, cursor, statuses, order_by)
List signers for a subaccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subaccount_id** | **uuid::Uuid** | Id representing the registered subaccount | [required] |
**order** | Option<**String**> | Direction to paginate through objects |  |
**limit** | Option<**f64**> | Limit the number of objects to return |  |
**cursor** | Option<**String**> | Pointer to the current object in pagination dataset |  |
**statuses** | Option<[**Vec<String>**](String.md)> | Filters signers by statuses |  |
**order_by** | Option<**String**> | Order by field |  |

### Return type

[**models::PageOfSignersDto**](PageOfSignersDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## linked_signer_controller_refresh_signer

> models::SignerDto linked_signer_controller_refresh_signer(refresh_linked_signer_dto)
Refreshes the expiry of a linked signer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refresh_linked_signer_dto** | [**RefreshLinkedSignerDto**](RefreshLinkedSignerDto.md) |  | [required] |

### Return type

[**models::SignerDto**](SignerDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## linked_signer_controller_revoke_signer

> models::SignerDto linked_signer_controller_revoke_signer(revoke_linked_signer_dto)
Revokes a signer address from a subaccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revoke_linked_signer_dto** | [**RevokeLinkedSignerDto**](RevokeLinkedSignerDto.md) |  | [required] |

### Return type

[**models::SignerDto**](SignerDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

