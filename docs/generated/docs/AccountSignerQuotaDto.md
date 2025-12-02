# AccountSignerQuotaDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_linked_signers_period_days** | **f64** | Ratelimit period in days for linking signers per account | 
**max_linked_signers_in_period** | **f64** | Max number of signer that can be linked within ratelimit period | 
**linked_signers_used_in_period** | **f64** | Number of signers linked within current ratelimit period | 
**linked_signer_refresh_hours_before_expiry** | **f64** | Hours before expiry when a signer can be refreshed | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


