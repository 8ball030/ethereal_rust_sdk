use crate::{
    apis::{
        Error,
        configuration::Configuration,
        product_api::{
            ProductControllerListError, ProductControllerListParams, product_controller_list,
        },
    },
    models::PageOfProductDtos,
};
pub struct ProductClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> ProductClient<'a> {
    pub fn list(
        &self,
        params: ProductControllerListParams,
    ) -> Result<PageOfProductDtos, Error<ProductControllerListError>> {
        product_controller_list(self.config, params)
    }
}