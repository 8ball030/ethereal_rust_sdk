use crate::{
    apis::{
        configuration::Configuration,
        rpc_api::{rpc_controller_get_config, RpcControllerGetConfigError},
        Error,
    },
    models::RpcConfigDto,
};
pub struct RpcClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> RpcClient<'a> {
    pub async fn get_config(&self) -> Result<RpcConfigDto, Error<RpcControllerGetConfigError>> {
        rpc_controller_get_config(self.config).await
    }
}
