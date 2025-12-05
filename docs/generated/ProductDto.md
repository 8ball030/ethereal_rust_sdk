# ProductDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Id representing the registered product | 
**ticker** | **String** | Product ticker based on the base quote token | 
**display_ticker** | **String** | Product display ticker based on the base quote token | 
**base_token_address** | **String** | Address of the base token (non-checksummed; zero address if virtual) | 
**quote_token_address** | **String** | Address of quote token (non-checksummed) | 
**base_token_name** | **String** | Name of the base token (e.g. BTC in BTCUSD) | 
**quote_token_name** | **String** | Name of the quote token (e.g. USD in BTCUSD) | 
**engine_type** | [**models::EngineType**](EngineType.md) |  | 
**onchain_id** | **i32** | The productId generated onchain after registering for the first time | 
**block_number** | **String** | Block number this product was registered on | 
**cumulative_funding_usd** | **String** | Cumulative funding in USD of the product (precision: 9) | 
**created_at** | **f64** | Product creation timestamp (ms since Unix Epoch) | 
**funding_updated_at** | Option<**f64**> | Unix timestamp when funding was last updated | [optional]
**min_quantity** | **String** | The minimum order quantity in native units expressed as a decimal (precision: 9) | 
**lot_size** | **String** | Quantity must be divisible by the lotSize in expressed as a decimal (precision: 9) | 
**tick_size** | **String** | Minimum price increment (tickSize) expressed as a decimal (precision: 9) | 
**maker_fee** | **String** | Fee charged to the maker on order trades expressed as a decimal (precision: 9) | 
**taker_fee** | **String** | Fee charged to the taker on order trades expressed as a decimal (precision: 9) | 
**max_quantity** | **String** | Max quantity per order in native units expressed as a decimal (precision: 9) | 
**min_price** | **String** | Min price in USD expressed as a decimal (precision: 9) | 
**max_price** | **String** | Max price in USD expressed as a decimal (precision: 9) | 
**volume24h** | **String** | 24h volume in base token native units expressed as a decimal (precision: 9) | 
**max_leverage** | **f64** | Maximum leverage allowed for the product | 
**pyth_feed_id** | **f64** | Pyth price feed id | 
**funding_rate1h** | **String** | Last computed hourly funding rate expressed as a decimal (precision: 9) | 
**open_interest** | **String** | OI of both sides in native units expressed as a decimal (precision: 9) | 
**max_open_interest_usd** | **String** | Max OI of one side in USD expressed as a decimal (precision: 9) | 
**max_position_notional_usd** | **String** | Max position notional value, in USD expressed as a decimal (precision: 9) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


