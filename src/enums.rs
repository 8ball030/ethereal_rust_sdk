use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
pub enum Environment {
    Mainnet,
    Testnet,
}

impl Environment {
    pub fn get_server_url(&self) -> &str {
        match self {
            Environment::Mainnet => "wss://ws2.ethereal.trade/v1/stream",
            Environment::Testnet => "wss://ws2.etherealtest.net/v1/stream",
        }
    }
}
