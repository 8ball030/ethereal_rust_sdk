# BookDepthMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timestamp** | **i64** | System timestamp in milliseconds when this BookDepth was emitted. | 
**previous_timestamp** | **i64** | Timestamp in ms of the previous BookDepth event for this product. | 
**product_id** | [**uuid::Uuid**](uuid::Uuid.md) | Identifier of the product. | 
**asks** | [**Vec<Vec<String>>**](Vec.md) | Array of ask levels as [price, quantity] string tuples. | 
**bids** | [**Vec<Vec<String>>**](Vec.md) | Array of bid levels as [price, quantity] string tuples. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


