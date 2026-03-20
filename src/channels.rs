use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Channels {
    PositionUpdate,
    TokenTransfer,
    L2Book,
    OrderFill,
    SubaccountLiquidation,
    OrderUpdate,
    TradeFill,
    Ticker,
}
impl Channels {
    pub fn as_string(&self) -> String {
        serde_json::to_string(self).unwrap().replace('"', "")
    }
}
