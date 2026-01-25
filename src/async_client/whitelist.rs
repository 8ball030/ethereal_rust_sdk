use crate::{
    apis::{
        configuration::Configuration,
        whitelist_api::{
            whitelist_controller_is_whitelisted, WhitelistControllerIsWhitelistedError,
            WhitelistControllerIsWhitelistedParams,
        },
        Error,
    },
    models::WhitelistDto,
};
pub struct WhitelistClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> WhitelistClient<'a> {
    pub async fn is_whitelisted(
        &self,
        params: WhitelistControllerIsWhitelistedParams,
    ) -> Result<WhitelistDto, Error<WhitelistControllerIsWhitelistedError>> {
        whitelist_controller_is_whitelisted(self.config, params).await
    }
}
