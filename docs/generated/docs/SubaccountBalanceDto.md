# SubaccountBalanceDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subaccount_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the subaccount | 
**token_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the token | 
**token_address** | **String** | ERC20 deposited token address (non-checksummed, zero address if virtual) | 
**token_name** | **String** | The unique exchange defined token name driven by addToken onchain | 
**amount** | **String** | Token balance in native units expressed as a decimal (precision: 9) | 
**available** | **String** | Portion of balance transferrable in native units expressed as a decimal (precision: 9) | 
**total_used** | **String** | Portion of balance non-transferrable in native units expressed as a decimal (precision: 9) | 
**updated_at** | **f64** | Token balance last updated timestamp (ms since Unix Epoch) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


