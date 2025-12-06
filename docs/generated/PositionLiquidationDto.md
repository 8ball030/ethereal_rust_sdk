# PositionLiquidationDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the position liquidation | 
**subaccount_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the subaccount that was liquidated | 
**product_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the product that was liquidated | 
**position_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the position that was liquidated | 
**liquidation_price** | **String** | Liquidation mark price in USD expressed as a decimal (precision: 9) | 
**cause** | [**models::PositionLiquidationCause**](PositionLiquidationCause.md) |  | 
**cost** | **String** | Position cost at the time of liquidation in USD expressed as a decimal (precision: 9) | 
**funding_charge_usd** | Option<**String**> | Funding charged in USD expressed as a decimal (precision: 9), undefined if not liquidated due to funding | [optional]
**position_side** | [**models::PositionSide**](PositionSide.md) |  | 
**created_at** | **f64** | Position liquidation timestamp (ms since Unix Epoch) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


