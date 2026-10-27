use crate::{
    apis::{
        configuration::Configuration,
        position_fee_api::{
            position_fee_controller_list_by_product_id,
            position_fee_controller_list_projected_fees, PositionFeeControllerListByProductIdError,
            PositionFeeControllerListByProductIdParams,
            PositionFeeControllerListProjectedFeesError,
            PositionFeeControllerListProjectedFeesParams,
        },
        Error,
    },
    models::{PageOfPositionFeeDtos, PageOfProjectedPositionFeeDtos},
};
pub struct PositionFeeClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> PositionFeeClient<'a> {
    pub async fn list_by_product_id(
        &self,
        params: PositionFeeControllerListByProductIdParams,
    ) -> Result<PageOfPositionFeeDtos, Error<PositionFeeControllerListByProductIdError>> {
        position_fee_controller_list_by_product_id(self.config, params).await
    }

    pub async fn list_projected_fees(
        &self,
        params: PositionFeeControllerListProjectedFeesParams,
    ) -> Result<PageOfProjectedPositionFeeDtos, Error<PositionFeeControllerListProjectedFeesError>>
    {
        position_fee_controller_list_projected_fees(self.config, params).await
    }
}
