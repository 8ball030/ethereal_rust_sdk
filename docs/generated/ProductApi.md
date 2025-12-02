# \ProductApi

All URIs are relative to *https://api.ethereal.trade*

Method | HTTP request | Description
------------- | ------------- | -------------
[**product_controller_get_by_id**](ProductApi.md#product_controller_get_by_id) | **GET** /v1/product/{id} | Returns product by id
[**product_controller_get_market_liquidity**](ProductApi.md#product_controller_get_market_liquidity) | **GET** /v1/product/market-liquidity | Returns the product market liquidity by id
[**product_controller_get_market_price**](ProductApi.md#product_controller_get_market_price) | **GET** /v1/product/market-price | Returns the product prices for an array of product ids
[**product_controller_list**](ProductApi.md#product_controller_list) | **GET** /v1/product | Returns a list of all products and its configuration



## product_controller_get_by_id

> models::ProductDto product_controller_get_by_id(id)
Returns product by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ProductDto**](ProductDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## product_controller_get_market_liquidity

> models::MarketLiquidityDto product_controller_get_market_liquidity(product_id)
Returns the product market liquidity by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **uuid::Uuid** | Id representing the registered product | [required] |

### Return type

[**models::MarketLiquidityDto**](MarketLiquidityDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## product_controller_get_market_price

> models::ListOfMarketPriceDtos product_controller_get_market_price(product_ids)
Returns the product prices for an array of product ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Array of product ids | [required] |

### Return type

[**models::ListOfMarketPriceDtos**](ListOfMarketPriceDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## product_controller_list

> models::PageOfProductDtos product_controller_list(order, limit, cursor, order_by, ticker)
Returns a list of all products and its configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order** | Option<**String**> | Direction to paginate through objects |  |
**limit** | Option<**f64**> | Limit the number of objects to return |  |
**cursor** | Option<**String**> | Pointer to the current object in pagination dataset |  |
**order_by** | Option<**String**> | Order by field |  |
**ticker** | Option<**String**> | Filter products by ticker (case insensitive) |  |

### Return type

[**models::PageOfProductDtos**](PageOfProductDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

