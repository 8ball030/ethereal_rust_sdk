use crate::{
    apis::{
        configuration::Configuration,
        usde_api::{
            usde_controller_list_transfers, UsdeControllerListTransfersError,
            UsdeControllerListTransfersParams,
        },
        Error,
    },
    models::PageOfUsdeTransfersDtos,
};
pub struct UsdeClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> UsdeClient<'a> {
    pub async fn list_transfers(
        &self,
        params: UsdeControllerListTransfersParams,
    ) -> Result<PageOfUsdeTransfersDtos, Error<UsdeControllerListTransfersError>> {
        usde_controller_list_transfers(self.config, params).await
    }
}
