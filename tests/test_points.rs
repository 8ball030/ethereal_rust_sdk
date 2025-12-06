mod common;
use ethereal_rust_sdk::apis::points_api::{
    PointsControllerListPointsPeriodsParams, PointsControllerListPointsSeasonSummariesParams,
};

#[test]
#[ignore]
fn test_list_points_periods() {
    let client = common::create_test_client().unwrap();
    let params = PointsControllerListPointsPeriodsParams::default();
    let result = client.points().list_points_periods(params);
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn test_list_points_season_summaries() {
    let client = common::create_test_client().unwrap();
    let params = PointsControllerListPointsSeasonSummariesParams::default();
    let result = client.points().list_points_season_summaries(params);
    assert!(result.is_ok());
}
