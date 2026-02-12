use crate::{
    apis::{
        configuration::Configuration,
        subaccount_api::{
            subaccount_controller_get_by_subaccount_id, subaccount_controller_list,
            subaccount_controller_list_by_account, subaccount_controller_list_subaccount_balances,
            SubaccountControllerGetBySubaccountIdError,
            SubaccountControllerGetBySubaccountIdParams, SubaccountControllerListByAccountError,
            SubaccountControllerListByAccountParams, SubaccountControllerListError,
            SubaccountControllerListParams, SubaccountControllerListSubaccountBalancesError,
            SubaccountControllerListSubaccountBalancesParams,
        },
        Error,
    },
    models::{PageOfSubaccountBalanceDtos, PageOfSubaccountDtos, SubaccountDto},
};
pub struct SubaccountClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> SubaccountClient<'a> {
    pub async fn get_by_subaccount_id(
        &self,
        params: SubaccountControllerGetBySubaccountIdParams,
    ) -> Result<SubaccountDto, Error<SubaccountControllerGetBySubaccountIdError>> {
        subaccount_controller_get_by_subaccount_id(self.config, params).await
    }

    pub async fn list(
        &self,
        params: SubaccountControllerListParams,
    ) -> Result<PageOfSubaccountDtos, Error<SubaccountControllerListError>> {
        subaccount_controller_list(self.config, params).await
    }

    pub async fn list_by_account(
        &self,
        params: SubaccountControllerListByAccountParams,
    ) -> Result<PageOfSubaccountDtos, Error<SubaccountControllerListByAccountError>> {
        subaccount_controller_list_by_account(self.config, params).await
    }

    pub async fn list_subaccount_balances(
        &self,
        params: SubaccountControllerListSubaccountBalancesParams,
    ) -> Result<PageOfSubaccountBalanceDtos, Error<SubaccountControllerListSubaccountBalancesError>>
    {
        subaccount_controller_list_subaccount_balances(self.config, params).await
    }
}
