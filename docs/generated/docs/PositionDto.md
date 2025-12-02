# PositionDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the position | 
**cost** | **String** | Current cost of the position in USD expressed as a decimal (precision: 9) | 
**size** | **String** | Position size in native units expressed as a decimal (precision: 9) | 
**funding_usd** | **String** | Charged but unapplied funding on position, negative if paid, expressed as a decimal (precision: 9) | 
**funding_accrued_usd** | **String** | Charged and applied funding on position, negative if paid, expressed as a decimal (precision: 9) | 
**fees_accrued_usd** | **String** | Fees accrued in USD expressed as a decimal (precision: 9) | 
**realized_pnl** | **String** | Realized PnL in USD expressed as a decimal (precision: 9) | 
**total_increase_notional** | **String** | Cumulative USD value of all position increases expressed as a decimal (precision: 9) | 
**total_increase_quantity** | **String** | Cumulative quantity of all position increases expressed as a decimal (precision: 9) | 
**total_decrease_notional** | **String** | Cumulative USD value of all position decreases expressed as a decimal (precision: 9) | 
**total_decrease_quantity** | **String** | Cumulative quantity of all position decreases expressed as a decimal (precision: 9) | 
**side** | **f64** | Side as either BUY (0) or SELL (1) | 
**product_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id of product to this position belongs to | 
**updated_at** | **f64** | Position last updated timestamp (ms since Unix Epoch) | 
**created_at** | **f64** | Position creation timestamp (ms since Unix Epoch) | 
**is_liquidated** | **bool** | Whether the position was liquidated | 
**liquidation_price** | Option<**String**> | Product price at the time of liquidation (undefined if not liquidated, precision: 9) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


