# SubmitOrderLimitDtoData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subaccount** | **String** | Bytes32 encoded subaccount name (0x prefix, zero padded) | 
**sender** | **String** | Address of account | 
**nonce** | **String** | Message nonce timestamp (nanoseconds since Unix Epoch) | 
**r#type** | [**models::OrderType**](OrderType.md) |  | 
**quantity** | [**rust_decimal::Decimal**](rust_decimal::Decimal.md) | Non-directional quantity of product in native units expressed as a decimal (precision: 9) | 
**side** | [**models::OrderSide**](OrderSide.md) |  | 
**onchain_id** | **i32** | Onchain generated productId from prior product registration | 
**engine_type** | [**models::EngineType**](EngineType.md) |  | 
**client_order_id** | Option<**String**> | A subaccount scoped unique client-generated order id (either a UUID or alphanumeric string up to 32 characters) | [optional]
**reduce_only** | Option<**bool**> | Whether this should be a reduce-only order, required for close | [optional][default to false]
**close** | Option<**bool**> | Order closes the entire current position, requires zero quantity and reduceOnly | [optional][default to false]
**stop_price** | Option<[**rust_decimal::Decimal**](rust_decimal::Decimal.md)> | Stop price expressed as a decimal (precision: 9), requires stopType | [optional][default to 0]
**stop_type** | Option<**f64**> | Stop type, either 0 (take-profit) or 1 (stop-loss), requires non-zero stopPrice | [optional]
**signed_at** | **i64** | Message signedAt current timestamp (seconds since Unix Epoch) | 
**expires_at** | Option<**i64**> | Order expiry timestamp (seconds since Unix Epoch), defaults to the maximum allowed value: signedAt + 6652800 | [optional]
**group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Group Id (UUID) for linking orders together in OCO/OTO relationships | [optional]
**group_contingency_type** | Option<**f64**> | Contingency type for order groups: OTO (Order-Triggers-Order) or OCO (One-Cancels-Other) | [optional]
**price** | [**rust_decimal::Decimal**](rust_decimal::Decimal.md) | Limit price expressed as a decimal (precision: 9) | 
**time_in_force** | **String** | How long an order will remain until executed/expired | [default to Gtd]
**post_only** | **bool** | Only add order if it does not immediately fill | [default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


