use crate::{
    apis::{
        Error,
        configuration::Configuration,
        whitelist_api::{
            WhitelistControllerIsWhitelistedError, WhitelistControllerIsWhitelistedParams,
            whitelist_controller_is_whitelisted,
        },
    },
    models::WhitelistDto,
};
pub struct WhitelistClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> WhitelistClient<'a> {
    pub fn is_whitelisted(
        &self,
        params: WhitelistControllerIsWhitelistedParams,
    ) -> Result<WhitelistDto, Error<WhitelistControllerIsWhitelistedError>> {
        whitelist_controller_is_whitelisted(self.config, params)
    }
}
