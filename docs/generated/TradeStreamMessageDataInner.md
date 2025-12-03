# TradeStreamMessageDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique identifier of the trade. | 
**price** | **String** | Trade price as a decimal string. | 
**filled** | **String** | Filled quantity as a decimal string. | 
**taker_side** | [**models::TakerSide**](TakerSide.md) |  | 
**created_at** | **f64** | Timestamp of when the trade was created (ms since Unix Epoch). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


