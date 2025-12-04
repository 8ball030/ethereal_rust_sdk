use crate::{
    apis::{
        configuration::Configuration,
    },
    enums::Environment,
    sync_client::product::ProductClient
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
}
