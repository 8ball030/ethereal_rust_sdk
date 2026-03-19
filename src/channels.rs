use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Channels {
    L2Book,
    OrderFill,
    OrderUpdate,
    PositionUpdate,
    SubaccountLiquidation,
    Ticker,
    TokenTransfer,
    TradeFill,
}
impl Channels {
    pub fn as_string(&self) -> String {
        serde_json::to_string(self).unwrap().replace('"', "")
    }
}
