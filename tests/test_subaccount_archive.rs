mod common;
use ethereal_rust_sdk::archive_apis::subaccount_archive_api::{
    SubaccountArchiveControllerGetTotalVolumeParams,
    SubaccountArchiveControllerListBalanceHistoryParams,
    SubaccountArchiveControllerListPositionFundingHistoryParams,
    SubaccountArchiveControllerListUnrealizedPnlHistoryParams,
    SubaccountArchiveControllerListVolumeHistoryParams,
};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::test]
async fn test_get_total_volume() {
    let client = common::create_test_client().await.unwrap();
    let params = SubaccountArchiveControllerGetTotalVolumeParams {
        subaccount_id: client.subaccounts.first().unwrap().id.clone().to_string(),
    };
    let result = client.subaccount_archive().get_total_volume(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_balance_history() {
    let client = common::create_test_client().await.unwrap();
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;
    let params = SubaccountArchiveControllerListBalanceHistoryParams {
        start_time: now_ms - 3_500_000.0, // 58m20s ago (safely within minute1 max range)
        resolution: "minute1".to_string(),
        subaccount_id: client.subaccounts.first().unwrap().id.clone().to_string(),
        ..Default::default()
    };
    let result = client
        .subaccount_archive()
        .list_balance_history(params)
        .await;
    if let Err(e) = &result {
        panic!("list_balance_history failed: {:?}", e);
    }
}

#[tokio::test]
async fn test_list_position_funding_history() {
    let client = common::create_test_client().await.unwrap();
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;
    let params = SubaccountArchiveControllerListPositionFundingHistoryParams {
        start_time: now_ms - 86_400_000.0, // 24 hours ago
        subaccount_id: client.subaccounts.first().unwrap().id.clone().to_string(),
        ..Default::default()
    };
    let result = client
        .subaccount_archive()
        .list_position_funding_history(params.clone())
        .await;
    if let Err(e) = &result {
        // attempt to fetch raw response for debugging
        let cfg = client.subaccount_archive().config;
        let start_time_str = params.start_time.to_string();
        let sub_id = params.subaccount_id.clone();
        let url = format!(
            "{}/v1/subaccount/funding?startTime={}&subaccountId={}",
            cfg.base_path, start_time_str, sub_id
        );
        let resp = cfg.client.get(&url).send().await;
        match resp {
            Ok(r) => {
                let status = r.status();
                let body = r
                    .text()
                    .await
                    .unwrap_or_else(|_| "<failed to read body>".to_string());
                panic!(
                    "list_position_funding_history failed: {:?}; raw status: {} body: {}",
                    e, status, body
                );
            }
            Err(reqe) => panic!(
                "list_position_funding_history failed: {:?}; request failed: {}",
                e, reqe
            ),
        }
    }
}

#[tokio::test]
async fn test_list_unrealized_pnl_history() {
    let client = common::create_test_client().await.unwrap();
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;
    let params = SubaccountArchiveControllerListUnrealizedPnlHistoryParams {
        start_time: now_ms - 86_400_000.0, // 24 hours ago
        resolution: "hour1".to_string(),   // unrealized PnL requires hourly or coarser
        subaccount_id: client.subaccounts.first().unwrap().id.clone().to_string(),
        ..Default::default()
    };
    let result = client
        .subaccount_archive()
        .list_unrealized_pnl_history(params)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_volume_history() {
    let client = common::create_test_client().await.unwrap();
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64;
    let params = SubaccountArchiveControllerListVolumeHistoryParams {
        start_time: now_ms - 3_500_000.0, // 58m20s ago (safely within minute1 max range)
        resolution: "minute1".to_string(),
        subaccount_id: client.subaccounts.first().unwrap().id.clone().to_string(),
        ..Default::default()
    };
    let result = client
        .subaccount_archive()
        .list_volume_history(params)
        .await;
    if let Err(e) = &result {
        panic!("list_volume_history failed: {:?}", e);
    }
}
