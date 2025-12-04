use crate::{
    apis::{
        Error,
        configuration::Configuration,
        token_api::{
            TokenControllerGetByIdError, TokenControllerGetByIdParams,
            TokenControllerInitiateWithdrawError, TokenControllerInitiateWithdrawParams,
            TokenControllerListError, TokenControllerListParams, TokenControllerListTransfersError,
            TokenControllerListTransfersParams, TokenControllerListWithdrawsError,
            TokenControllerListWithdrawsParams, token_controller_get_by_id,
            token_controller_initiate_withdraw, token_controller_list,
            token_controller_list_transfers, token_controller_list_withdraws,
        },
    },
    models::{PageOfTokensDtos, PageOfTransfersDtos, PageOfWithdrawDtos, TokenDto, WithdrawDto},
};
pub struct TokenClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> TokenClient<'a> {
    pub fn get_by_id(
        &self,
        params: TokenControllerGetByIdParams,
    ) -> Result<TokenDto, Error<TokenControllerGetByIdError>> {
        token_controller_get_by_id(self.config, params)
    }

    pub fn initiate_withdraw(
        &self,
        params: TokenControllerInitiateWithdrawParams,
    ) -> Result<WithdrawDto, Error<TokenControllerInitiateWithdrawError>> {
        token_controller_initiate_withdraw(self.config, params)
    }

    pub fn list(
        &self,
        params: TokenControllerListParams,
    ) -> Result<PageOfTokensDtos, Error<TokenControllerListError>> {
        token_controller_list(self.config, params)
    }

    pub fn list_transfers(
        &self,
        params: TokenControllerListTransfersParams,
    ) -> Result<PageOfTransfersDtos, Error<TokenControllerListTransfersError>> {
        token_controller_list_transfers(self.config, params)
    }

    pub fn list_withdraws(
        &self,
        params: TokenControllerListWithdrawsParams,
    ) -> Result<PageOfWithdrawDtos, Error<TokenControllerListWithdrawsError>> {
        token_controller_list_withdraws(self.config, params)
    }
}
