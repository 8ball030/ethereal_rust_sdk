use crate::{
    apis::{
        configuration::Configuration,
        product_api::{
            product_controller_get_by_id, product_controller_get_market_liquidity,
            product_controller_get_market_price, product_controller_list,
            ProductControllerGetByIdError, ProductControllerGetByIdParams,
            ProductControllerGetMarketLiquidityError, ProductControllerGetMarketLiquidityParams,
            ProductControllerGetMarketPriceError, ProductControllerGetMarketPriceParams,
            ProductControllerListError, ProductControllerListParams,
        },
        Error,
    },
    models::{ListOfMarketPriceDtos, MarketLiquidityDto, PageOfProductDtos, ProductDto},
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
}
