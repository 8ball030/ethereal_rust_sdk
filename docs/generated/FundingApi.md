# \FundingApi

All URIs are relative to *https://api.ethereal.trade*

Method | HTTP request | Description
------------- | ------------- | -------------
[**funding_controller_get_projected_funding_rate**](FundingApi.md#funding_controller_get_projected_funding_rate) | **GET** /v1/funding/projected | Returns the projected funding rate for a product. Deprecated, use `listProjectedRates` instead.
[**funding_controller_list_by_product_id**](FundingApi.md#funding_controller_list_by_product_id) | **GET** /v1/funding | Returns a list funding rates for a product over a time period
[**funding_controller_list_projected_rates**](FundingApi.md#funding_controller_list_projected_rates) | **GET** /v1/funding/projected-rate | Returns a list of projected funding rates for the given products



## funding_controller_get_projected_funding_rate

> models::ProjectedFundingDto funding_controller_get_projected_funding_rate(product_id)
Returns the projected funding rate for a product. Deprecated, use `listProjectedRates` instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **uuid::Uuid** | Id representing the registered product | [required] |

### Return type

[**models::ProjectedFundingDto**](ProjectedFundingDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## funding_controller_list_by_product_id

> models::PageOfFundingDtos funding_controller_list_by_product_id(product_id, range, order, limit, cursor, order_by)
Returns a list funding rates for a product over a time period

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **uuid::Uuid** | Id representing the registered product | [required] |
**range** | **String** | The range of time of funding rates to retrieve | [required] |
**order** | Option<**String**> | Direction to paginate through objects |  |
**limit** | Option<**f64**> | Limit the number of objects to return |  |
**cursor** | Option<**String**> | Pointer to the current object in pagination dataset |  |
**order_by** | Option<**String**> | Order by field |  |

### Return type

[**models::PageOfFundingDtos**](PageOfFundingDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## funding_controller_list_projected_rates

> models::PageOfProjectedFundingDtos funding_controller_list_projected_rates(product_ids)
Returns a list of projected funding rates for the given products

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | Array of product ids | [required] |

### Return type

[**models::PageOfProjectedFundingDtos**](PageOfProjectedFundingDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

