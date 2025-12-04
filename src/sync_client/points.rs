use crate::{
    apis::{
        Error,
        configuration::Configuration,
        points_api::{
            PointsControllerListPointsPeriodsError, PointsControllerListPointsPeriodsParams,
            PointsControllerListPointsSeasonSummariesError,
            PointsControllerListPointsSeasonSummariesParams, points_controller_list_points_periods,
            points_controller_list_points_season_summaries,
        },
    },
    models::{ListOfPointsPeriodDtos, ListOfPointsSeasonSummariesDtos},
};
pub struct PointsClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> PointsClient<'a> {
    pub fn list_points_periods(
        &self,
        params: PointsControllerListPointsPeriodsParams,
    ) -> Result<ListOfPointsPeriodDtos, Error<PointsControllerListPointsPeriodsError>> {
        points_controller_list_points_periods(self.config, params)
    }

    pub fn list_points_season_summaries(
        &self,
        params: PointsControllerListPointsSeasonSummariesParams,
    ) -> Result<
        ListOfPointsSeasonSummariesDtos,
        Error<PointsControllerListPointsSeasonSummariesError>,
    > {
        points_controller_list_points_season_summaries(self.config, params)
    }
}
