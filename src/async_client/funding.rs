use crate::{
    apis::{
        configuration::Configuration,
        funding_api::{
            funding_controller_list_by_product_id, funding_controller_list_projected_rates,
            FundingControllerListByProductIdError, FundingControllerListByProductIdParams,
            FundingControllerListProjectedRatesError, FundingControllerListProjectedRatesParams,
        },
        Error,
    },
    models::{PageOfFundingDtos, PageOfProjectedFundingDtos},
};
pub struct FundingClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> FundingClient<'a> {
    pub async fn list_by_product_id(
        &self,
        params: FundingControllerListByProductIdParams,
    ) -> Result<PageOfFundingDtos, Error<FundingControllerListByProductIdError>> {
        funding_controller_list_by_product_id(self.config, params).await
    }

    pub async fn list_projected_rates(
        &self,
        params: FundingControllerListProjectedRatesParams,
    ) -> Result<PageOfProjectedFundingDtos, Error<FundingControllerListProjectedRatesError>> {
        funding_controller_list_projected_rates(self.config, params).await
    }
}
