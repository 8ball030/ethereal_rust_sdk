# MarketLiquidityDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timestamp** | **f64** | Most recent book update, created timestamp if never updated (ms since Unix Epoch) | 
**previous_timestamp** | Option<**f64**> | Previous book update, undefined if never updated (ms since Unix Epoch) | [optional]
**product_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the product | 
**asks** | [**Vec<Vec<serde_json::Value>>**](Vec.md) | An array of ask tuple pairs (price, quantity) ordered in asc | 
**bids** | [**Vec<Vec<serde_json::Value>>**](Vec.md) | An array of bid tuple pairs (price, quantity) ordered in desc | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


