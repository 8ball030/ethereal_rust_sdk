# TradeDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the trade | 
**taker_order_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id of the taker order | 
**maker_order_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id of the maker order | 
**taker_client_order_id** | Option<**String**> | Client-generated order id of the taker order | [optional]
**maker_client_order_id** | Option<**String**> | Client-generated order id of the maker order | [optional]
**maker_fee_usd** | **String** | Maker fee in USD expressed as a decimal (precision: 9) | 
**taker_fee_usd** | **String** | Taker fee in USD expressed as a decimal (precision: 9) | 
**price** | **String** | Price expressed as a decimal (precision: 9) | 
**filled** | **String** | Quantity filled in native units expressed as a decimal (precision: 9) | 
**maker_side** | **f64** | Maker side as either BUY (0) or SELL (1) | 
**taker_side** | **f64** | Taker side as either BUY (0) or SELL (1) | 
**product_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id of product the trade was made against | 
**created_at** | **f64** | Trade creation timestamp (ms since Unix Epoch) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


