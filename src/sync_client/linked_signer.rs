use crate::{
    apis::{
        Error,
        configuration::Configuration,
        linked_signer_api::{
            LinkedSignerControllerGetAccountQuotaError,
            LinkedSignerControllerGetAccountQuotaParams, LinkedSignerControllerGetSignerError,
            LinkedSignerControllerGetSignerParams, LinkedSignerControllerLinkSignerError,
            LinkedSignerControllerLinkSignerParams, LinkedSignerControllerListBySubaccountIdError,
            LinkedSignerControllerListBySubaccountIdParams,
            LinkedSignerControllerRefreshSignerError, LinkedSignerControllerRefreshSignerParams,
            LinkedSignerControllerRevokeSignerError, LinkedSignerControllerRevokeSignerParams,
            linked_signer_controller_get_account_quota, linked_signer_controller_get_signer,
            linked_signer_controller_link_signer, linked_signer_controller_list_by_subaccount_id,
            linked_signer_controller_refresh_signer, linked_signer_controller_revoke_signer,
        },
    },
    models::{AccountSignerQuotaDto, PageOfSignersDto, SignerDto},
};
pub struct LinkedSignerClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> LinkedSignerClient<'a> {
    pub fn get_account_quota(
        &self,
        params: LinkedSignerControllerGetAccountQuotaParams,
    ) -> Result<AccountSignerQuotaDto, Error<LinkedSignerControllerGetAccountQuotaError>> {
        linked_signer_controller_get_account_quota(self.config, params)
    }

    pub fn get_signer(
        &self,
        params: LinkedSignerControllerGetSignerParams,
    ) -> Result<SignerDto, Error<LinkedSignerControllerGetSignerError>> {
        linked_signer_controller_get_signer(self.config, params)
    }

    pub fn link_signer(
        &self,
        params: LinkedSignerControllerLinkSignerParams,
    ) -> Result<SignerDto, Error<LinkedSignerControllerLinkSignerError>> {
        linked_signer_controller_link_signer(self.config, params)
    }

    pub fn list_by_subaccount_id(
        &self,
        params: LinkedSignerControllerListBySubaccountIdParams,
    ) -> Result<PageOfSignersDto, Error<LinkedSignerControllerListBySubaccountIdError>> {
        linked_signer_controller_list_by_subaccount_id(self.config, params)
    }

    pub fn refresh_signer(
        &self,
        params: LinkedSignerControllerRefreshSignerParams,
    ) -> Result<SignerDto, Error<LinkedSignerControllerRefreshSignerError>> {
        linked_signer_controller_refresh_signer(self.config, params)
    }

    pub fn revoke_signer(
        &self,
        params: LinkedSignerControllerRevokeSignerParams,
    ) -> Result<SignerDto, Error<LinkedSignerControllerRevokeSignerError>> {
        linked_signer_controller_revoke_signer(self.config, params)
    }
}
