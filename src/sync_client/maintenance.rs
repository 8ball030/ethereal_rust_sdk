use crate::{
    apis::{
        Error,
        configuration::Configuration,
        maintenance_api::{
            MaintenanceControllerIsMaintenanceError, maintenance_controller_is_maintenance,
        },
    },
    models::MaintenanceDto,
};
pub struct MaintenanceClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> MaintenanceClient<'a> {
    pub fn is_maintenance(
        &self,
    ) -> Result<MaintenanceDto, Error<MaintenanceControllerIsMaintenanceError>> {
        maintenance_controller_is_maintenance(self.config)
    }
}
