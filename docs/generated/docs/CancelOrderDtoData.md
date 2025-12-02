# CancelOrderDtoData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subaccount** | **String** | Bytes32 encoded subaccount name (0x prefix, zero padded) | 
**sender** | **String** | Account or linked signer address that canceled this order | 
**nonce** | **String** | Message nonce timestamp (nanoseconds since Unix Epoch) | 
**order_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Ids of the orders to be canceled (clientOrderIds combined length cannot exceed maximum) | [optional]
**client_order_ids** | Option<**Vec<String>**> | Client-generated order ids to be canceled (orderIds combined length cannot exceed maximum) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


