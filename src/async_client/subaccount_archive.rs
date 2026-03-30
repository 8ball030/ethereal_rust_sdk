use crate::{
    archive_apis::{
        configuration::Configuration,
        subaccount_archive_api::{
            subaccount_archive_controller_get_total_volume,
            subaccount_archive_controller_list_balance_history,
            subaccount_archive_controller_list_position_funding_history,
            subaccount_archive_controller_list_unrealized_pnl_history,
            subaccount_archive_controller_list_volume_history,
            SubaccountArchiveControllerGetTotalVolumeError,
            SubaccountArchiveControllerGetTotalVolumeParams,
            SubaccountArchiveControllerListBalanceHistoryError,
            SubaccountArchiveControllerListBalanceHistoryParams,
            SubaccountArchiveControllerListPositionFundingHistoryError,
            SubaccountArchiveControllerListPositionFundingHistoryParams,
            SubaccountArchiveControllerListUnrealizedPnlHistoryError,
            SubaccountArchiveControllerListUnrealizedPnlHistoryParams,
            SubaccountArchiveControllerListVolumeHistoryError,
            SubaccountArchiveControllerListVolumeHistoryParams,
        },
        Error,
    },
    archive_models::{
        PageOfBalanceHistoryDtos, PageOfPositionFundingHistoryDtos,
        PageOfSubaccountVolumeHistoryDtos, PageOfUnrealizedPnlHistoryDtos,
        TotalSubaccountVolumeDto,
    },
};
pub struct SubaccountArchiveClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> SubaccountArchiveClient<'a> {
    pub async fn get_total_volume(
        &self,
        params: SubaccountArchiveControllerGetTotalVolumeParams,
    ) -> Result<TotalSubaccountVolumeDto, Error<SubaccountArchiveControllerGetTotalVolumeError>>
    {
        subaccount_archive_controller_get_total_volume(self.config, params).await
    }

    pub async fn list_balance_history(
        &self,
        params: SubaccountArchiveControllerListBalanceHistoryParams,
    ) -> Result<PageOfBalanceHistoryDtos, Error<SubaccountArchiveControllerListBalanceHistoryError>>
    {
        subaccount_archive_controller_list_balance_history(self.config, params).await
    }

    pub async fn list_position_funding_history(
        &self,
        params: SubaccountArchiveControllerListPositionFundingHistoryParams,
    ) -> Result<
        PageOfPositionFundingHistoryDtos,
        Error<SubaccountArchiveControllerListPositionFundingHistoryError>,
    > {
        subaccount_archive_controller_list_position_funding_history(self.config, params).await
    }

    pub async fn list_unrealized_pnl_history(
        &self,
        params: SubaccountArchiveControllerListUnrealizedPnlHistoryParams,
    ) -> Result<
        PageOfUnrealizedPnlHistoryDtos,
        Error<SubaccountArchiveControllerListUnrealizedPnlHistoryError>,
    > {
        subaccount_archive_controller_list_unrealized_pnl_history(self.config, params).await
    }

    pub async fn list_volume_history(
        &self,
        params: SubaccountArchiveControllerListVolumeHistoryParams,
    ) -> Result<
        PageOfSubaccountVolumeHistoryDtos,
        Error<SubaccountArchiveControllerListVolumeHistoryError>,
    > {
        subaccount_archive_controller_list_volume_history(self.config, params).await
    }
}
