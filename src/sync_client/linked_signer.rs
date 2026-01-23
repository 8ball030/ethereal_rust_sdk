use crate::{
    apis::{
        configuration::Configuration,
        linked_signer_api::{
            linked_signer_controller_get_account_quota, linked_signer_controller_get_signer,
            linked_signer_controller_link_signer, linked_signer_controller_list_by_subaccount_id,
            linked_signer_controller_refresh_signer, linked_signer_controller_revoke_signer,
            LinkedSignerControllerGetAccountQuotaError,
            LinkedSignerControllerGetAccountQuotaParams, LinkedSignerControllerGetSignerError,
            LinkedSignerControllerGetSignerParams, LinkedSignerControllerLinkSignerError,
            LinkedSignerControllerLinkSignerParams, LinkedSignerControllerListBySubaccountIdError,
            LinkedSignerControllerListBySubaccountIdParams,
            LinkedSignerControllerRefreshSignerError, LinkedSignerControllerRefreshSignerParams,
            LinkedSignerControllerRevokeSignerError, LinkedSignerControllerRevokeSignerParams,
        },
        Error,
    },
    models::{AccountSignerQuotaDto, PageOfSignersDto, SignerDto},
};
pub struct LinkedSignerClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> LinkedSignerClient<'a> {
    pub async fn get_account_quota(
        &self,
        params: LinkedSignerControllerGetAccountQuotaParams,
    ) -> Result<AccountSignerQuotaDto, Error<LinkedSignerControllerGetAccountQuotaError>> {
        linked_signer_controller_get_account_quota(self.config, params).await
    }

    pub async fn get_signer(
        &self,
        params: LinkedSignerControllerGetSignerParams,
    ) -> Result<SignerDto, Error<LinkedSignerControllerGetSignerError>> {
        linked_signer_controller_get_signer(self.config, params).await
    }

    pub async fn link_signer(
        &self,
        params: LinkedSignerControllerLinkSignerParams,
    ) -> Result<SignerDto, Error<LinkedSignerControllerLinkSignerError>> {
        linked_signer_controller_link_signer(self.config, params).await
    }

    pub async fn list_by_subaccount_id(
        &self,
        params: LinkedSignerControllerListBySubaccountIdParams,
    ) -> Result<PageOfSignersDto, Error<LinkedSignerControllerListBySubaccountIdError>> {
        linked_signer_controller_list_by_subaccount_id(self.config, params).await
    }

    pub async fn refresh_signer(
        &self,
        params: LinkedSignerControllerRefreshSignerParams,
    ) -> Result<SignerDto, Error<LinkedSignerControllerRefreshSignerError>> {
        linked_signer_controller_refresh_signer(self.config, params).await
    }

    pub async fn revoke_signer(
        &self,
        params: LinkedSignerControllerRevokeSignerParams,
    ) -> Result<SignerDto, Error<LinkedSignerControllerRevokeSignerError>> {
        linked_signer_controller_revoke_signer(self.config, params).await
    }
}
