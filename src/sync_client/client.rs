use crate::{
    apis::{configuration::Configuration, subaccount_api::SubaccountControllerListByAccountParams},
    enums::Environment,
    models::SubaccountDto,
    sync_client::{
        funding::FundingClient, linked_signer::LinkedSignerClient, maintenance::MaintenanceClient,
        order::OrderClient, points::PointsClient, position::PositionClient, product::ProductClient,
        referral::ReferralClient, rpc::RpcClient, subaccount::SubaccountClient, time::TimeClient,
        token::TokenClient, whitelist::WhitelistClient,
    },
};

use ethers::signers::{LocalWallet, Signer};

fn get_server_url(environment: &Environment) -> &str {
    match environment {
        Environment::Mainnet => "https://api.ethereal.trade",
        Environment::Testnet => "https://api.etherealtest.net",
    }
}

pub struct HttpClient {
    config: Configuration,
    pub wallet: LocalWallet,
    pub address: String,
    pub subaccounts: Vec<SubaccountDto>,
}

impl HttpClient {
    pub fn new(env: Environment, private_key: &str) -> Self {
        let config = Configuration {
            base_path: get_server_url(&env).to_string(),
            ..Default::default()
        };
        let wallet = private_key.parse::<LocalWallet>().unwrap();
        let address = format!("{:?}", wallet.address());
        let subaccounts = SubaccountClient { config: &config }
            .list_by_account(SubaccountControllerListByAccountParams {
                sender: address.clone(),
                ..Default::default()
            })
            .unwrap()
            .data;
        Self {
            config,
            wallet,
            address,
            subaccounts,
        }
    }

    pub fn product(&self) -> ProductClient<'_> {
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
