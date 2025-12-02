# ReferralDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the referral | 
**referee** | **String** | Address of the referee (non-checksummed) | 
**referrer** | Option<**String**> | Address of the referrer (non-checksummed; undefined if not set) | [optional]
**code** | Option<**String**> | The referee's referral code to be shared and claimed by others (undefined if not activated) | [optional]
**code_usage_remaining** | **f64** | Number of remaining times the referral code can be claimed | 
**referee_total_points** | **String** | Total points (excl. referral points) earned by the referee since referee claimed the code | 
**created_at** | **f64** | Timestamp of when this referral was activated (ms since Unix Epoch) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


