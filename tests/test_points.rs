mod common;
use ethereal_rust_sdk::apis::points_api::{
    PointsControllerListPointsPeriodsParams, PointsControllerListPointsSeasonSummariesParams,
};

#[tokio::test]
#[ignore]
async fn test_list_points_periods() {
    let client = common::create_test_client().await.unwrap();
    let params = PointsControllerListPointsPeriodsParams::default();
    let result = client.points().list_points_periods(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_list_points_season_summaries() {
    let client = common::create_test_client().await.unwrap();
    let params = PointsControllerListPointsSeasonSummariesParams::default();
    let result = client.points().list_points_season_summaries(params).await;
    assert!(result.is_ok());
}
