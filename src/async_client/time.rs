use crate::{
    apis::{
        Error,
        configuration::Configuration,
        time_api::{
            TimeControllerGetSystemTimeError, TimeControllerPostSystemTimeError,
            time_controller_get_system_time, time_controller_post_system_time,
        },
    },
    models::SystemTimeDto,
};
pub struct TimeClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> TimeClient<'a> {
    pub async fn get_system_time(
        &self,
    ) -> Result<SystemTimeDto, Error<TimeControllerGetSystemTimeError>> {
        time_controller_get_system_time(self.config).await
    }

    pub async fn post_system_time(
        &self,
    ) -> Result<SystemTimeDto, Error<TimeControllerPostSystemTimeError>> {
        time_controller_post_system_time(self.config).await
    }
}
