use crate::{
    apis::{
        configuration::Configuration,
        referral_api::{
            referral_controller_activate, referral_controller_claim_code,
            referral_controller_get_summary, referral_controller_list_referrals,
            ReferralControllerActivateError, ReferralControllerActivateParams,
            ReferralControllerClaimCodeError, ReferralControllerClaimCodeParams,
            ReferralControllerGetSummaryError, ReferralControllerGetSummaryParams,
            ReferralControllerListReferralsError, ReferralControllerListReferralsParams,
        },
        Error,
    },
    models::{PageOfReferralDtos, ReferralDto, ReferralSummaryDto},
};
pub struct ReferralClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> ReferralClient<'a> {
    pub async fn activate(
        &self,
        params: ReferralControllerActivateParams,
    ) -> Result<ReferralDto, Error<ReferralControllerActivateError>> {
        referral_controller_activate(self.config, params).await
    }

    pub async fn claim_code(
        &self,
        params: ReferralControllerClaimCodeParams,
    ) -> Result<ReferralDto, Error<ReferralControllerClaimCodeError>> {
        referral_controller_claim_code(self.config, params).await
    }

    pub async fn get_summary(
        &self,
        params: ReferralControllerGetSummaryParams,
    ) -> Result<ReferralSummaryDto, Error<ReferralControllerGetSummaryError>> {
        referral_controller_get_summary(self.config, params).await
    }

    pub async fn list_referrals(
        &self,
        params: ReferralControllerListReferralsParams,
    ) -> Result<PageOfReferralDtos, Error<ReferralControllerListReferralsError>> {
        referral_controller_list_referrals(self.config, params).await
    }
}
