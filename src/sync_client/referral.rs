use crate::{
    apis::{
        Error,
        configuration::Configuration,
        referral_api::{
            ReferralControllerActivateError, ReferralControllerActivateParams,
            ReferralControllerClaimCodeError, ReferralControllerClaimCodeParams,
            ReferralControllerGetCodeUsageError, ReferralControllerGetCodeUsageParams,
            ReferralControllerGetSummaryError, ReferralControllerGetSummaryParams,
            ReferralControllerListReferralsError, ReferralControllerListReferralsParams,
            referral_controller_activate, referral_controller_claim_code,
            referral_controller_get_code_usage, referral_controller_get_summary,
            referral_controller_list_referrals,
        },
    },
    models::{PageOfReferralDtos, ReferralCodeUsageDto, ReferralDto},
};
pub struct ReferralClient<'a> {
    pub config: &'a Configuration,
}

impl<'a> ReferralClient<'a> {
    pub fn activate(
        &self,
        params: ReferralControllerActivateParams,
    ) -> Result<ReferralDto, Error<ReferralControllerActivateError>> {
        referral_controller_activate(self.config, params)
    }

    pub fn claim_code(
        &self,
        params: ReferralControllerClaimCodeParams,
    ) -> Result<ReferralDto, Error<ReferralControllerClaimCodeError>> {
        referral_controller_claim_code(self.config, params)
    }

    pub fn get_code_usage(
        &self,
        params: ReferralControllerGetCodeUsageParams,
    ) -> Result<ReferralCodeUsageDto, Error<ReferralControllerGetCodeUsageError>> {
        referral_controller_get_code_usage(self.config, params)
    }

    pub fn get_summary(
        &self,
        params: ReferralControllerGetSummaryParams,
    ) -> Result<ReferralDto, Error<ReferralControllerGetSummaryError>> {
        referral_controller_get_summary(self.config, params)
    }

    pub fn list_referrals(
        &self,
        params: ReferralControllerListReferralsParams,
    ) -> Result<PageOfReferralDtos, Error<ReferralControllerListReferralsError>> {
        referral_controller_list_referrals(self.config, params)
    }
}
