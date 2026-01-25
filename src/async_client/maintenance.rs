use crate::{
    apis::{
        configuration::Configuration,
        maintenance_api::{
            maintenance_controller_is_maintenance, MaintenanceControllerIsMaintenanceError,
        },
        Error,
    },
    models::MaintenanceDto,
};
pub struct MaintenanceClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> MaintenanceClient<'a> {
    pub async fn is_maintenance(
        &self,
    ) -> Result<MaintenanceDto, Error<MaintenanceControllerIsMaintenanceError>> {
        maintenance_controller_is_maintenance(self.config).await
    }
}
