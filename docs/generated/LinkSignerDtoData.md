# LinkSignerDtoData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subaccount_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the registered subaccount | 
**sender** | **String** | Account address that created the signature in this message | 
**subaccount** | **String** | Bytes32 encoded subaccount name (0x prefix, zero padded) | 
**signer** | **String** | Address of signer to allowed for delegated signing | 
**nonce** | **String** | Message nonce timestamp (nanoseconds since Unix Epoch) | 
**signed_at** | **i32** | Message signedAt current timestamp (seconds since Unix Epoch) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


