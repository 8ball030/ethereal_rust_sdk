# SubmitOrderCreatedDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the created order | 
**client_order_id** | Option<**String**> | A subaccount scoped unique client-generated order id (either a UUID or alphanumeric string up to 32 characters) | [optional]
**filled** | **String** | Filled amount in native units expressed as a decimal (precision: 9) | 
**result** | **String** | Code indicating the result of the submission | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


