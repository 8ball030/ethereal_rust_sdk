# \PointsApi

All URIs are relative to *https://api.ethereal.trade*

Method | HTTP request | Description
------------- | ------------- | -------------
[**points_controller_list_points_periods**](PointsApi.md#points_controller_list_points_periods) | **GET** /v1/points | Returns a list of points periods for a given address and season
[**points_controller_list_points_season_summaries**](PointsApi.md#points_controller_list_points_season_summaries) | **GET** /v1/points/summary | Returns a list of points season summaries for a given address



## points_controller_list_points_periods

> models::ListOfPointsPeriodDtos points_controller_list_points_periods(address, season, epoch)
Returns a list of points periods for a given address and season

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Address of account | [required] |
**season** | **f64** | Season number | [required] |
**epoch** | **f64** | Epoch number within the season | [required] |

### Return type

[**models::ListOfPointsPeriodDtos**](ListOfPointsPeriodDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## points_controller_list_points_season_summaries

> models::ListOfPointsSeasonSummariesDtos points_controller_list_points_season_summaries(address)
Returns a list of points season summaries for a given address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Address of account | [required] |

### Return type

[**models::ListOfPointsSeasonSummariesDtos**](ListOfPointsSeasonSummariesDtos.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

