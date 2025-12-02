# \PositionApi

All URIs are relative to *https://api.ethereal.trade*

Method | HTTP request | Description
------------- | ------------- | -------------
[**position_controller_get_active**](PositionApi.md#position_controller_get_active) | **GET** /v1/position/active | Returns the active position for a subaccount and product
[**position_controller_get_by_id**](PositionApi.md#position_controller_get_by_id) | **GET** /v1/position/{id} | Returns position by id
[**position_controller_list_by_subaccount_id**](PositionApi.md#position_controller_list_by_subaccount_id) | **GET** /v1/position | Returns a filtered list of positions for a given subaccount
[**position_controller_list_fills_by_position_id**](PositionApi.md#position_controller_list_fills_by_position_id) | **GET** /v1/position/fill | Returns a filtered list of fills for a given position
[**position_controller_list_liquidations_by_subaccount_id**](PositionApi.md#position_controller_list_liquidations_by_subaccount_id) | **GET** /v1/position/liquidation | Returns a list of liquidations



## position_controller_get_active

> models::PositionDto position_controller_get_active(subaccount_id, product_id)
Returns the active position for a subaccount and product

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subaccount_id** | **uuid::Uuid** | Id representing the registered subaccount | [required] |
**product_id** | **uuid::Uuid** | Id of product to filter position by | [required] |

### Return type

[**models::PositionDto**](PositionDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## position_controller_get_by_id

> models::PositionDto position_controller_get_by_id(id)
Returns position by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::PositionDto**](PositionDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## position_controller_list_by_subaccount_id

> models::PageOfPositionDtos position_controller_list_by_subaccount_id(subaccount_id, order, limit, cursor, product_ids, open, order_by, created_after, created_before, side, is_liquidated)
Returns a filtered list of positions for a given subaccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subaccount_id** | **uuid::Uuid** | Id representing the registered subaccount | [required] |
**order** | Option<**String**> | Direction to paginate through objects |  |
**limit** | Option<**f64**> | Limit the number of objects to return |  |
**cursor** | Option<**String**> | Pointer to the current object in pagination dataset |  |
**product_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Array of product ids to filter for |  |
**open** | Option<**bool**> | Include or exclude open positions (i.e. non-zero size) |  |
**order_by** | Option<**String**> | Order by field |  |
**created_after** | Option<**f64**> | Filter by order fills created before timestamp exclusive (ms since Unix epoch) |  |
**created_before** | Option<**f64**> | Filter by order fills created before timestamp inclusive (ms since Unix epoch) |  |
**side** | Option<**f64**> | Side as either BUY (0) or SELL (1) |  |
**is_liquidated** | Option<**bool**> | Filter by liquidated positions |  |

### Return type

[**models::PageOfPositionDtos**](PageOfPositionDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## position_controller_list_fills_by_position_id

> models::PageOfPositionFillDtos position_controller_list_fills_by_position_id(position_id, order, limit, cursor, order_by)
Returns a filtered list of fills for a given position

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**position_id** | **uuid::Uuid** | Id of the position to filter fills by | [required] |
**order** | Option<**String**> | Direction to paginate through objects |  |
**limit** | Option<**f64**> | Limit the number of objects to return |  |
**cursor** | Option<**String**> | Pointer to the current object in pagination dataset |  |
**order_by** | Option<**String**> | Order by field |  |

### Return type

[**models::PageOfPositionFillDtos**](PageOfPositionFillDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## position_controller_list_liquidations_by_subaccount_id

> models::PageOfPositionLiquidationsDto position_controller_list_liquidations_by_subaccount_id(order, limit, cursor, order_by)
Returns a list of liquidations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order** | Option<**String**> | Direction to paginate through objects |  |
**limit** | Option<**f64**> | Limit the number of objects to return |  |
**cursor** | Option<**String**> | Pointer to the current object in pagination dataset |  |
**order_by** | Option<**String**> | Order by field |  |

### Return type

[**models::PageOfPositionLiquidationsDto**](PageOfPositionLiquidationsDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

