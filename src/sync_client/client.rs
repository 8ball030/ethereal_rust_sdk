use crate::{
    apis::configuration::Configuration, enums::Environment, sync_client::funding::FundingClient,
    sync_client::linked_signer::LinkedSignerClient, sync_client::maintenance::MaintenanceClient,
    sync_client::order::OrderClient, sync_client::points::PointsClient,
    sync_client::position::PositionClient, sync_client::product::ProductClient,
    sync_client::referral::ReferralClient, sync_client::rpc::RpcClient,
    sync_client::subaccount::SubaccountClient, sync_client::time::TimeClient,
    sync_client::token::TokenClient, sync_client::whitelist::WhitelistClient,
};

fn get_server_url(environment: &Environment) -> &str {
    match environment {
        Environment::Production => "https://api.ethereal.trade",
        Environment::Testnet => "https://api.etherealtest.net",
    }
}

pub struct HttpClient {
    config: Configuration,
}

impl HttpClient {
    pub fn new(env: Environment) -> Self {
        let config = Configuration {
            base_path: get_server_url(&env).to_string(),
            ..Default::default()
        };
        Self { config }
    }

    pub fn products(&self) -> ProductClient<'_> {
        ProductClient {
            config: &self.config,
        }
    }
    pub fn funding(&self) -> FundingClient<'_> {
        FundingClient {
            config: &self.config,
        }
    }
    pub fn linked_signer(&self) -> LinkedSignerClient<'_> {
        LinkedSignerClient {
            config: &self.config,
        }
    }
    pub fn maintenance(&self) -> MaintenanceClient<'_> {
        MaintenanceClient {
            config: &self.config,
        }
    }
    pub fn order(&self) -> OrderClient<'_> {
        OrderClient {
            config: &self.config,
        }
    }
    pub fn points(&self) -> PointsClient<'_> {
        PointsClient {
            config: &self.config,
        }
    }
    pub fn position(&self) -> PositionClient<'_> {
        PositionClient {
            config: &self.config,
        }
    }
    pub fn referral(&self) -> ReferralClient<'_> {
        ReferralClient {
            config: &self.config,
        }
    }
    pub fn rpc(&self) -> RpcClient<'_> {
        RpcClient {
            config: &self.config,
        }
    }

    pub fn subaccount(&self) -> SubaccountClient<'_> {
        SubaccountClient {
            config: &self.config,
        }
    }
    pub fn time(&self) -> TimeClient<'_> {
        TimeClient {
            config: &self.config,
        }
    }
    pub fn token(&self) -> TokenClient<'_> {
        TokenClient {
            config: &self.config,
        }
    }
    pub fn whitelist(&self) -> WhitelistClient<'_> {
        WhitelistClient {
            config: &self.config,
        }
    }
}
