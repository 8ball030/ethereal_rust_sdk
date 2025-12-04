use crate::{
    apis::{
        Error,
        configuration::Configuration,
        order_api::{
            OrderControllerCancelError, OrderControllerCancelParams, OrderControllerDryRunError,
            OrderControllerDryRunParams, OrderControllerGetByIdError, OrderControllerGetByIdParams,
            OrderControllerListBySubaccountIdError, OrderControllerListBySubaccountIdParams,
            OrderControllerListFillsBySubaccountIdError,
            OrderControllerListFillsBySubaccountIdParams, OrderControllerListTradesError,
            OrderControllerListTradesParams, OrderControllerSubmitError,
            OrderControllerSubmitParams, order_controller_cancel, order_controller_dry_run,
            order_controller_get_by_id, order_controller_list_by_subaccount_id,
            order_controller_list_fills_by_subaccount_id, order_controller_list_trades,
            order_controller_submit,
        },
    },
    models::{
        DryRunOrderCreatedDto, ListOfCancelOrderResultDtos, OrderDto, PageOfOrderDtos,
        PageOfOrderFillDtos, PageOfTradeDtos, SubmitOrderCreatedDto,
    },
};
pub struct OrderClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> OrderClient<'a> {
    pub fn cancel(
        &self,
        params: OrderControllerCancelParams,
    ) -> Result<ListOfCancelOrderResultDtos, Error<OrderControllerCancelError>> {
        order_controller_cancel(self.config, params)
    }

    pub fn dry_run(
        &self,
        params: OrderControllerDryRunParams,
    ) -> Result<DryRunOrderCreatedDto, Error<OrderControllerDryRunError>> {
        order_controller_dry_run(self.config, params)
    }

    pub fn get_by_id(
        &self,
        params: OrderControllerGetByIdParams,
    ) -> Result<OrderDto, Error<OrderControllerGetByIdError>> {
        order_controller_get_by_id(self.config, params)
    }

    pub fn list_by_subaccount_id(
        &self,
        params: OrderControllerListBySubaccountIdParams,
    ) -> Result<PageOfOrderDtos, Error<OrderControllerListBySubaccountIdError>> {
        order_controller_list_by_subaccount_id(self.config, params)
    }

    pub fn list_fills_by_subaccount_id(
        &self,
        params: OrderControllerListFillsBySubaccountIdParams,
    ) -> Result<PageOfOrderFillDtos, Error<OrderControllerListFillsBySubaccountIdError>> {
        order_controller_list_fills_by_subaccount_id(self.config, params)
    }

    pub fn list_trades(
        &self,
        params: OrderControllerListTradesParams,
    ) -> Result<PageOfTradeDtos, Error<OrderControllerListTradesError>> {
        order_controller_list_trades(self.config, params)
    }

    pub fn submit(
        &self,
        params: OrderControllerSubmitParams,
    ) -> Result<SubmitOrderCreatedDto, Error<OrderControllerSubmitError>> {
        order_controller_submit(self.config, params)
    }
}
