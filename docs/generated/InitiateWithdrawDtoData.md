# InitiateWithdrawDtoData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account** | **String** | Account address to withdraw from | 
**subaccount** | **String** | Bytes32 encoded subaccount name (0x prefix, zero padded) | 
**token** | **String** | Address of token to be withdrawn | 
**amount** | **String** | Amount to withdraw in native units expressed as a decimal (precision: 9) | 
**lz_destination_address** | **String** | Bytes32 encoded LayerZero destination address (with 0x prefix, left zero padded) | 
**lz_destination_eid** | **f64** | LayerZero destination endpoint ID for the transfer (zero if not bridging) | 
**nonce** | **String** | Message nonce timestamp (nanoseconds since Unix Epoch) | 
**signed_at** | **i64** | Message signedAt current timestamp (seconds since Unix Epoch) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


