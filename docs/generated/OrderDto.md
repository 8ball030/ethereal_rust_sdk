# OrderDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the order | 
**client_order_id** | Option<**String**> | A subaccount scoped unique client-generated order id (either a UUID or alphanumeric string up to 32 characters) | [optional]
**r#type** | **String** | Trade order type e.g. MARKET or LIMIT | 
**available_quantity** | **String** | Remaining quantity (if modified or reduced) in native units expressed as a decimal (precision: 9) | 
**quantity** | **String** | Original quantity (as per order submission) in native units expressed as a decimal (precision: 9) | 
**side** | [**models::OrderSide**](OrderSide.md) |  | 
**product_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id of product this order was placed against | 
**subaccount_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id of the subaccount associated to order | 
**status** | [**models::OrderStatus**](OrderStatus.md) |  | 
**reduce_only** | **bool** | Indicates if the order is reduce only | 
**close** | **bool** | Order closes the entire current position | 
**updated_at** | **f64** | Order last updated timestamp (ms since Unix Epoch) | 
**created_at** | **f64** | Order creation timestamp (ms since Unix Epoch) | 
**sender** | **String** | Account or linked signer address that placed this order | 
**price** | **String** | Limit price in native units expressed as a decimal, zero if market order (precision: 9) | 
**filled** | **String** | Filled amount in native units expressed as a decimal (precision: 9) | 
**stop_price** | **String** | Stop price expressed as a decimal (zero if not a stop order, precision: 9) | 
**stop_type** | Option<**f64**> | Side as either BUY (0) or SELL (1) | [optional]
**stop_price_type** | Option<**f64**> | Type of stop price (stop orders only) | [optional]
**time_in_force** | Option<**String**> | How long an order will remain until executed/expired (required if limit) | [optional]
**expires_at** | **f64** | Order expiry timestamp (seconds since Unix Epoch) | 
**post_only** | Option<**bool**> | Only add order if it does not immediately fill (limit only) | [optional]
**group_contingency_type** | Option<**f64**> | Type of OTOCO relationship (OTO or OCO) | [optional]
**group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Id of the group this order belongs to | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


