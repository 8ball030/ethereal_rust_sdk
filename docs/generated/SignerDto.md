# SignerDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Id representing the linked signer | 
**signer** | **String** | Address of the signer linked with the subaccount (non-checksummed) | 
**status** | **String** | Status of the signer | 
**block_number** | Option<**String**> | Block number the signer has been linked on. Undefined means it has not be processed | [optional]
**linked_at** | Option<**f64**> | Onchain linkage timestamp (ms since Unix Epoch) | [optional]
**revoked_block_number** | Option<**String**> | Block number the signer has been revoked on. Undefined means it has not be processed | [optional]
**revoked_at** | Option<**f64**> | Onchain revocation timestamp (ms since Unix Epoch) | [optional]
**expires_at** | **f64** | Signer expiry timestamp (ms since Unix Epoch) | 
**created_at** | **f64** | Link signer submission timestamp (ms since Unix Epoch) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


