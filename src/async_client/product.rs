use crate::{
    apis::{
        configuration::Configuration,
        product_api::{
            product_controller_get_by_id, product_controller_get_market_liquidity,
            product_controller_get_market_price, product_controller_list,
            product_controller_list_mark_price_gaps, ProductControllerGetByIdError,
            ProductControllerGetByIdParams, ProductControllerGetMarketLiquidityError,
            ProductControllerGetMarketLiquidityParams, ProductControllerGetMarketPriceError,
            ProductControllerGetMarketPriceParams, ProductControllerListError,
            ProductControllerListMarkPriceGapsError, ProductControllerListMarkPriceGapsParams,
            ProductControllerListParams,
        },
        Error,
    },
    models::{
        ListOfMarketPriceDtos, MarketLiquidityDto, PageOfMarkPriceGapDtos, PageOfProductDtos,
        ProductDto,
    },
};
pub struct ProductClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> ProductClient<'a> {
    pub async fn get_by_id(
        &self,
        params: ProductControllerGetByIdParams,
    ) -> Result<ProductDto, Error<ProductControllerGetByIdError>> {
        product_controller_get_by_id(self.config, params).await
    }

    pub async fn get_market_liquidity(
        &self,
        params: ProductControllerGetMarketLiquidityParams,
    ) -> Result<MarketLiquidityDto, Error<ProductControllerGetMarketLiquidityError>> {
        product_controller_get_market_liquidity(self.config, params).await
    }

    pub async fn get_market_price(
        &self,
        params: ProductControllerGetMarketPriceParams,
    ) -> Result<ListOfMarketPriceDtos, Error<ProductControllerGetMarketPriceError>> {
        product_controller_get_market_price(self.config, params).await
    }

    pub async fn list(
        &self,
        params: ProductControllerListParams,
    ) -> Result<PageOfProductDtos, Error<ProductControllerListError>> {
        product_controller_list(self.config, params).await
    }

    pub async fn list_mark_price_gaps(
        &self,
        params: ProductControllerListMarkPriceGapsParams,
    ) -> Result<PageOfMarkPriceGapDtos, Error<ProductControllerListMarkPriceGapsError>> {
        product_controller_list_mark_price_gaps(self.config, params).await
    }
}
