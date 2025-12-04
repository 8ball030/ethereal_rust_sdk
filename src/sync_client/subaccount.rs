use crate::{
    apis::{
        Error,
        configuration::Configuration,
        subaccount_api::{
            SubaccountControllerGetBySubaccountIdError,
            SubaccountControllerGetBySubaccountIdParams, SubaccountControllerListByAccountError,
            SubaccountControllerListByAccountParams,
            SubaccountControllerListSubaccountBalancesError,
            SubaccountControllerListSubaccountBalancesParams,
            subaccount_controller_get_by_subaccount_id, subaccount_controller_list_by_account,
            subaccount_controller_list_subaccount_balances,
        },
    },
    models::{PageOfSubaccountBalanceDtos, PageOfSubaccountDtos, SubaccountDto},
};
pub struct SubaccountClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> SubaccountClient<'a> {
    pub fn get_by_subaccount_id(
        &self,
        params: SubaccountControllerGetBySubaccountIdParams,
    ) -> Result<SubaccountDto, Error<SubaccountControllerGetBySubaccountIdError>> {
        subaccount_controller_get_by_subaccount_id(self.config, params)
    }

    pub fn list_by_account(
        &self,
        params: SubaccountControllerListByAccountParams,
    ) -> Result<PageOfSubaccountDtos, Error<SubaccountControllerListByAccountError>> {
        subaccount_controller_list_by_account(self.config, params)
    }

    pub fn list_subaccount_balances(
        &self,
        params: SubaccountControllerListSubaccountBalancesParams,
    ) -> Result<PageOfSubaccountBalanceDtos, Error<SubaccountControllerListSubaccountBalancesError>>
    {
        subaccount_controller_list_subaccount_balances(self.config, params)
    }
}
