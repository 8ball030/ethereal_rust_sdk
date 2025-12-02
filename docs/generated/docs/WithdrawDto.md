# WithdrawDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the withdraw | 
**initiated_block_number** | Option<**String**> | Block number the withdraw was initiated on | [optional]
**finalized_block_number** | Option<**String**> | Block number the withdraw was completed on | [optional]
**status** | **String** | Current status of the withdraw | 
**subaccount** | **String** | Bytes32 encoded subaccount name (0x prefix, zero padded) | 
**token** | **String** | Address of asset to withdraw (non-checksummed) | 
**lz_destination_address** | Option<**String**> | LayerZero destination address (leading 0x bytes32 encoded) for the transfer (if withdraw) | [optional]
**lz_destination_eid** | Option<**f64**> | LayerZero destination endpoint ID for the transfer (if withdraw) | [optional]
**amount** | **String** | Amount of asset transferred expressed as a decimal | 
**created_at** | **f64** | Withdraw creation timestamp (ms since Unix Epoch) | 
**withdraw_digest** | **String** | Bytes32 hash of the withdraw data (with 0x prefix) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


