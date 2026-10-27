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
            Environment::Mainnet => "wss://ws.meridian.xyz/v1/stream",
            Environment::Testnet => "wss://ws.meridiantest.net/v1/stream",
        }
    }
}
