# OrderFillDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Id of the fill | 
**order_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id of the order (from the context of the specified subaccount) | 
**client_order_id** | Option<**String**> | A subaccount scoped unique client-generated order id (either a UUID or alphanumeric string up to 32 characters) | [optional]
**price** | **String** | Fill price in expressed as a decimal (precision: 9) | 
**filled** | **String** | Quantity filled in native units expressed as a decimal (precision: 9) | 
**r#type** | [**models::OrderType**](OrderType.md) |  | 
**side** | [**models::OrderSide**](OrderSide.md) |  | 
**reduce_only** | **bool** | Indicates if the fill is reduce only | 
**fee_usd** | **String** | The provided subaccount's charged fee in USD expressed as a decimal (precision: 9) | 
**is_maker** | **bool** | Indicates if the fill was a maker or taker | 
**product_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id of product the order fill was made against | 
**subaccount_id** | **String** | Id of the subaccount associated to order fill | 
**created_at** | **f64** | Fill creation timestamp (ms since Unix Epoch) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


