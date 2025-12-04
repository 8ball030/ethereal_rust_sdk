use crate::{
    apis::{
        Error,
        configuration::Configuration,
        position_api::{
            PositionControllerGetActiveError, PositionControllerGetActiveParams,
            PositionControllerGetByIdError, PositionControllerGetByIdParams,
            PositionControllerListBySubaccountIdError, PositionControllerListBySubaccountIdParams,
            PositionControllerListFillsByPositionIdError,
            PositionControllerListFillsByPositionIdParams,
            PositionControllerListLiquidationsBySubaccountIdError,
            PositionControllerListLiquidationsBySubaccountIdParams, position_controller_get_active,
            position_controller_get_by_id, position_controller_list_by_subaccount_id,
            position_controller_list_fills_by_position_id,
            position_controller_list_liquidations_by_subaccount_id,
        },
    },
    models::{
        PageOfPositionDtos, PageOfPositionFillDtos, PageOfPositionLiquidationsDto, PositionDto,
    },
};
pub struct PositionClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> PositionClient<'a> {
    pub fn get_active(
        &self,
        params: PositionControllerGetActiveParams,
    ) -> Result<PositionDto, Error<PositionControllerGetActiveError>> {
        position_controller_get_active(self.config, params)
    }

    pub fn get_by_id(
        &self,
        params: PositionControllerGetByIdParams,
    ) -> Result<PositionDto, Error<PositionControllerGetByIdError>> {
        position_controller_get_by_id(self.config, params)
    }

    pub fn list_by_subaccount_id(
        &self,
        params: PositionControllerListBySubaccountIdParams,
    ) -> Result<PageOfPositionDtos, Error<PositionControllerListBySubaccountIdError>> {
        position_controller_list_by_subaccount_id(self.config, params)
    }

    pub fn list_fills_by_position_id(
        &self,
        params: PositionControllerListFillsByPositionIdParams,
    ) -> Result<PageOfPositionFillDtos, Error<PositionControllerListFillsByPositionIdError>> {
        position_controller_list_fills_by_position_id(self.config, params)
    }

    pub fn list_liquidations_by_subaccount_id(
        &self,
        params: PositionControllerListLiquidationsBySubaccountIdParams,
    ) -> Result<
        PageOfPositionLiquidationsDto,
        Error<PositionControllerListLiquidationsBySubaccountIdError>,
    > {
        position_controller_list_liquidations_by_subaccount_id(self.config, params)
    }
}
