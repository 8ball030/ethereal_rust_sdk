# \OrderApi

All URIs are relative to *https://api.ethereal.trade*

Method | HTTP request | Description
------------- | ------------- | -------------
[**order_controller_cancel**](OrderApi.md#order_controller_cancel) | **POST** /v1/order/cancel | Cancels one or more orders given an array of order ids
[**order_controller_dry_run**](OrderApi.md#order_controller_dry_run) | **POST** /v1/order/dry-run | Submits a dry-mode to simulate an order submission
[**order_controller_get_by_id**](OrderApi.md#order_controller_get_by_id) | **GET** /v1/order/{id} | Returns an order by their id
[**order_controller_list_by_subaccount_id**](OrderApi.md#order_controller_list_by_subaccount_id) | **GET** /v1/order | Returns a filtered array of orders by the subaccount
[**order_controller_list_fills_by_subaccount_id**](OrderApi.md#order_controller_list_fills_by_subaccount_id) | **GET** /v1/order/fill | Returns a filtered array of order fills
[**order_controller_list_trades**](OrderApi.md#order_controller_list_trades) | **GET** /v1/order/trade | Returns a filtered array of trades
[**order_controller_submit**](OrderApi.md#order_controller_submit) | **POST** /v1/order | Place an order for trading



## order_controller_cancel

> models::ListOfCancelOrderResultDtos order_controller_cancel(cancel_order_dto)
Cancels one or more orders given an array of order ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cancel_order_dto** | [**CancelOrderDto**](CancelOrderDto.md) |  | [required] |

### Return type

[**models::ListOfCancelOrderResultDtos**](ListOfCancelOrderResultDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_controller_dry_run

> models::DryRunOrderCreatedDto order_controller_dry_run(submit_dry_order_dto)
Submits a dry-mode to simulate an order submission

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_dry_order_dto** | [**SubmitDryOrderDto**](SubmitDryOrderDto.md) |  | [required] |

### Return type

[**models::DryRunOrderCreatedDto**](DryRunOrderCreatedDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_controller_get_by_id

> models::OrderDto order_controller_get_by_id(id)
Returns an order by their id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::OrderDto**](OrderDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_controller_list_by_subaccount_id

> models::PageOfOrderDtos order_controller_list_by_subaccount_id(subaccount_id, order, limit, cursor, client_order_id, product_ids, created_after, created_before, side, close, stop_types, is_working, is_pending, order_by)
Returns a filtered array of orders by the subaccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subaccount_id** | **uuid::Uuid** | Id of the subaccount to query for | [required] |
**order** | Option<**String**> | Direction to paginate through objects |  |
**limit** | Option<**f64**> | Limit the number of objects to return |  |
**cursor** | Option<**String**> | Pointer to the current object in pagination dataset |  |
**client_order_id** | Option<**String**> | Client-generated order id to query for (either a valid UUID or alphanumeric string up to 32 characters) |  |
**product_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Array of product ids to filter for |  |
**created_after** | Option<**f64**> | Filter by orders created after timestamp exclusive (ms since Unix epoch) |  |
**created_before** | Option<**f64**> | Filter by orders created before timestamp inclusive (ms since Unix epoch) |  |
**side** | Option<**f64**> | Side of the order to filter for |  |
**close** | Option<**bool**> | Whether the order is a position close order |  |
**stop_types** | Option<[**Vec<f64>**](f64.md)> | Array of StopTypes to filter by |  |
**is_working** | Option<**bool**> | Filter by orders that are working: NEW, FILLED_PARTIAL |  |
**is_pending** | Option<**bool**> | Filter by orders that are pending |  |
**order_by** | Option<**String**> | Order by field |  |

### Return type

[**models::PageOfOrderDtos**](PageOfOrderDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_controller_list_fills_by_subaccount_id

> models::PageOfOrderFillDtos order_controller_list_fills_by_subaccount_id(subaccount_id, order, limit, cursor, product_ids, created_after, created_before, side, order_by, include_self_trades)
Returns a filtered array of order fills

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subaccount_id** | **uuid::Uuid** | Id of the subaccount to filter fills by | [required] |
**order** | Option<**String**> | Direction to paginate through objects |  |
**limit** | Option<**f64**> | Limit the number of objects to return |  |
**cursor** | Option<**String**> | Pointer to the current object in pagination dataset |  |
**product_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Array of product ids to filter for |  |
**created_after** | Option<**f64**> | Filter by order fills created before timestamp exclusive (ms since Unix epoch) |  |
**created_before** | Option<**f64**> | Filter by order fills created before timestamp inclusive (ms since Unix epoch) |  |
**side** | Option<**f64**> | Side as either BUY (0) or SELL (1) |  |
**order_by** | Option<**String**> | Order by field |  |
**include_self_trades** | Option<**bool**> | Explicitly include self trades (excluded by default) |  |[default to false]

### Return type

[**models::PageOfOrderFillDtos**](PageOfOrderFillDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_controller_list_trades

> models::PageOfTradeDtos order_controller_list_trades(product_id, order, limit, cursor, order_by)
Returns a filtered array of trades

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **uuid::Uuid** | Id of the product to filter trades by | [required] |
**order** | Option<**String**> | Direction to paginate through objects |  |
**limit** | Option<**f64**> | Limit the number of objects to return |  |
**cursor** | Option<**String**> | Pointer to the current object in pagination dataset |  |
**order_by** | Option<**String**> | Order by fields |  |

### Return type

[**models::PageOfTradeDtos**](PageOfTradeDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## order_controller_submit

> models::SubmitOrderCreatedDto order_controller_submit(submit_order_dto)
Place an order for trading

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_order_dto** | [**SubmitOrderDto**](SubmitOrderDto.md) |  | [required] |

### Return type

[**models::SubmitOrderCreatedDto**](SubmitOrderCreatedDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

