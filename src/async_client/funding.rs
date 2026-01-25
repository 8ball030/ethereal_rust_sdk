use crate::{
    apis::{
        configuration::Configuration,
        funding_api::{
            funding_controller_get_projected_funding_rate, funding_controller_list_by_product_id,
            funding_controller_list_projected_rates, FundingControllerGetProjectedFundingRateError,
            FundingControllerGetProjectedFundingRateParams, FundingControllerListByProductIdError,
            FundingControllerListByProductIdParams, FundingControllerListProjectedRatesError,
            FundingControllerListProjectedRatesParams,
        },
        Error,
    },
    models::{PageOfFundingDtos, PageOfProjectedFundingDtos, ProjectedFundingDto},
};
pub struct FundingClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> FundingClient<'a> {
    pub async fn get_projected_funding_rate(
        &self,
        params: FundingControllerGetProjectedFundingRateParams,
    ) -> Result<ProjectedFundingDto, Error<FundingControllerGetProjectedFundingRateError>> {
        funding_controller_get_projected_funding_rate(self.config, params).await
    }

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
