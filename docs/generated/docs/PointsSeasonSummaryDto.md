# PointsSeasonSummaryDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the summary of points in this season | 
**address** | **String** | Address of the account (non-checksummed) | 
**season** | **f64** | Season number | 
**total_points** | **String** | Total points earned including referral points in this season expressed as a decimal (precision: 9) | 
**previous_total_points** | **String** | Total points earned before most recent distribution expressed as a decimal (precision: 9) | 
**referral_points** | **String** | Referral points earned expressed as a decimal (precision: 9) | 
**previous_referral_points** | **String** | Referral points earned before most recent distribution expressed as a decimal (precision: 9) | 
**rank** | **f64** | Current rank in this season | 
**previous_rank** | **f64** | Rank before most recent distribution | 
**tier** | **f64** | Account tier derived based on activity this season | 
**created_at** | **f64** | Points season summary creation timestamp (ms since Unix Epoch) | 
**updated_at** | **f64** | Points season summary last update timestamp (ms since Unix Epoch) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


