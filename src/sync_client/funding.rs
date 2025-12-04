use crate::{
    apis::{
        Error,
        configuration::Configuration,
        funding_api::{
            FundingControllerGetProjectedFundingRateError,
            FundingControllerGetProjectedFundingRateParams, FundingControllerListByProductIdError,
            FundingControllerListByProductIdParams, FundingControllerListProjectedRatesError,
            FundingControllerListProjectedRatesParams,
            funding_controller_get_projected_funding_rate, funding_controller_list_by_product_id,
            funding_controller_list_projected_rates,
        },
    },
    models::{PageOfFundingDtos, PageOfProjectedFundingDtos, ProjectedFundingDto},
};
pub struct FundingClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> FundingClient<'a> {
    pub fn get_projected_funding_rate(
        &self,
        params: FundingControllerGetProjectedFundingRateParams,
    ) -> Result<ProjectedFundingDto, Error<FundingControllerGetProjectedFundingRateError>> {
        funding_controller_get_projected_funding_rate(self.config, params)
    }

    pub fn list_by_product_id(
        &self,
        params: FundingControllerListByProductIdParams,
    ) -> Result<PageOfFundingDtos, Error<FundingControllerListByProductIdError>> {
        funding_controller_list_by_product_id(self.config, params)
    }

    pub fn list_projected_rates(
        &self,
        params: FundingControllerListProjectedRatesParams,
    ) -> Result<PageOfProjectedFundingDtos, Error<FundingControllerListProjectedRatesError>> {
        funding_controller_list_projected_rates(self.config, params)
    }
}
