# SubaccountDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the registered subaccount | 
**name** | **String** | Bytes32 encoded subaccount name (0x prefix, zero padded) | 
**account** | **String** | Address of the account which registered the subaccount (non-checksummed) | 
**created_block_number** | **String** | Block number this subaccount was created on | 
**registered_block_number** | Option<**String**> | Block number this subaccount was registered on | [optional]
**created_at** | **f64** | Subaccount creation timestamp (ms since Unix Epoch) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


