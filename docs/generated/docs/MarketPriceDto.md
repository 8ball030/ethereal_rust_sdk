# MarketPriceDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the registered product | 
**best_bid_price** | Option<**String**> | Best bid price in USD expressed as a decimal (precision: 9) | [optional]
**best_ask_price** | Option<**String**> | Best ask price in USD expressed as a decimal (precision: 9) | [optional]
**oracle_price** | Option<**String**> | Oracle price in USD expressed as a decimal (precision: 9) | [optional]
**price24h_ago** | Option<**String**> | Price of product 24hrs ago in USD expressed as a decimal (precision: 9) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


