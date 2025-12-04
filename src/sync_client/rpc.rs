use crate::{
    apis::{
        Error,
        configuration::Configuration,
        rpc_api::{RpcControllerGetConfigError, rpc_controller_get_config},
    },
    models::RpcConfigDto,
};
pub struct RpcClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> RpcClient<'a> {
    pub fn get_config(&self) -> Result<RpcConfigDto, Error<RpcControllerGetConfigError>> {
        rpc_controller_get_config(self.config)
    }
}
