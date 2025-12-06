use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
pub enum Environment {
    Mainnet,
    Testnet,
}
