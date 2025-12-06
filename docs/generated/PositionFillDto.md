# PositionFillDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**price** | **String** | Fill price expressed as a decimal (precision: 9) | 
**filled** | **String** | Quantity filled in native units expressed as a decimal (precision: 9) | 
**realized_pnl** | **String** | Realized PnL from the fill in USD expressed as a decimal (precision: 9) | 
**r#type** | **String** | Corresponding order type that led to the position fill, LIQUIDATED if takeover | 
**side** | [**models::PositionSide**](PositionSide.md) |  | 
**reduce_only** | **bool** | Indicates if the fill is reduce only | 
**fee_usd** | **String** | The charged fee in USD expressed as a decimal (precision: 9) | 
**created_at** | **f64** | Fill creation timestamp (ms since Unix Epoch) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


