# PointsPeriodDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the points period entry | 
**address** | **String** | Address of the account (non-checksummed) | 
**season** | **f64** | Season number | 
**epoch** | **f64** | Epoch number within the season | 
**points** | **String** | Points earned in this epoch expressed as a decimal (precision: 9) | 
**referral_points** | **String** | Referral points earned in this epoch expressed as a decimal (precision: 9) | 
**started_at** | **f64** | Beginning of points period (ms since Unix Epoch) | 
**ended_at** | **f64** | End of points period (ms since Unix Epoch) | 
**created_at** | **f64** | Points period creation timestamp (ms since Unix Epoch) | 
**updated_at** | **f64** | Points period last update timestamp (ms since Unix Epoch) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


