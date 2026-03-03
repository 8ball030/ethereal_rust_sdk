use crate::{
    apis::{
        Error,
        configuration::Configuration,
        rate_limit_api::{RateLimitControllerGetConfigError, rate_limit_controller_get_config},
    },
    models::RateLimitConfigResponseDto,
};
pub struct RateLimitClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> RateLimitClient<'a> {
    pub async fn get_config(
        &self,
    ) -> Result<RateLimitConfigResponseDto, Error<RateLimitControllerGetConfigError>> {
        rate_limit_controller_get_config(self.config).await
    }
}
