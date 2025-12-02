# TransferDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the transfer | 
**initiated_block_number** | Option<**String**> | Block number the transfer was initiated on | [optional]
**finalized_block_number** | Option<**String**> | Block number the transfer was completed on | [optional]
**status** | **String** | Current status of the transfer | 
**subaccount_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the registered subaccount | 
**token_name** | **String** | The unique exchange defined token name driven by addToken onchain | 
**token_address** | **String** | Address of token transferred (non-checksummed) | 
**r#type** | **String** | Type of transfer (WITHDRAW or DEPOSIT) | 
**amount** | **String** | Amount of tokens transferred in native units expressed as a decimal (precision: 9) | 
**lz_destination_address** | Option<**String**> | LayerZero destination address (leading 0x bytes32 encoded) for the transfer (if withdraw) | [optional]
**lz_destination_eid** | Option<**f64**> | LayerZero destination endpoint ID for the transfer (if withdraw) | [optional]
**fee** | **String** | Fee paid for the transfer in native units expressed as a decimal (precision: 9) | 
**created_at** | **f64** | Transfer creation timestamp (ms since Unix Epoch) | 
**initiated_transaction_hash** | Option<**String**> | Transaction hash for the initiation of the transfer | [optional]
**finalized_transaction_hash** | Option<**String**> | Transaction hash for the finalization of the transfer | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


