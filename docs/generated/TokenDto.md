# TokenDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the token | 
**address** | **String** | Address of the token (non-checksummed; zero address if virtual) | 
**lz_oft_address** | **String** | LayerZero OFT address of the token (non-checksummed; if has LZ OFT or adapter, zero address if not) | 
**name** | **String** | The unique exchange defined token name driven by addToken onchain | 
**erc20_name** | Option<**String**> | ERC20 token name (available if not virtual) | [optional]
**erc20_symbol** | Option<**String**> | ERC20 token symbol (available if not virtual) | [optional]
**erc20_decimals** | Option<**f64**> | ERC20 token decimals (available if not virtual) | [optional]
**deposit_enabled** | **bool** | Whether the token is enabled for deposit | 
**withdraw_enabled** | **bool** | Whether the token is enabled for withdraw | 
**deposit_fee** | **String** | Amount of native units charged on deposit expressed as a decimal (precision: 9) | 
**withdraw_fee** | **String** | Amount of native units charged on withdraw expressed as a decimal (precision: 9) | 
**min_deposit** | **String** | Min deposit amount as native units expressed as a decimal (precision: 9) | 
**created_at** | **f64** | Token creation timestamp (ms since Unix Epoch) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


