use crate::{
    apis::{
        configuration::Configuration,
        points_api::{
            points_controller_get_total_points, points_controller_list_points_periods,
            points_controller_list_points_season_summaries, PointsControllerGetTotalPointsError,
            PointsControllerGetTotalPointsParams, PointsControllerListPointsPeriodsError,
            PointsControllerListPointsPeriodsParams,
            PointsControllerListPointsSeasonSummariesError,
            PointsControllerListPointsSeasonSummariesParams,
        },
        Error,
    },
    models::{ListOfPointsPeriodDtos, ListOfPointsSeasonSummariesDtos, TotalPointsDto},
};
pub struct PointsClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> PointsClient<'a> {
    pub async fn get_total_points(
        &self,
        params: PointsControllerGetTotalPointsParams,
    ) -> Result<TotalPointsDto, Error<PointsControllerGetTotalPointsError>> {
        points_controller_get_total_points(self.config, params).await
    }

    pub async fn list_points_periods(
        &self,
        params: PointsControllerListPointsPeriodsParams,
    ) -> Result<ListOfPointsPeriodDtos, Error<PointsControllerListPointsPeriodsError>> {
        points_controller_list_points_periods(self.config, params).await
    }

    pub async fn list_points_season_summaries(
        &self,
        params: PointsControllerListPointsSeasonSummariesParams,
    ) -> Result<
        ListOfPointsSeasonSummariesDtos,
        Error<PointsControllerListPointsSeasonSummariesError>,
    > {
        points_controller_list_points_season_summaries(self.config, params).await
    }
}
