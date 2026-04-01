use std::future::Future;

use bytes::Bytes;

use crate::{
    channels::Channels,
    models::{
        L2BookMessage, OrderFillMessage, OrderUpdateMessage, PositionUpdateMessage,
        SubaccountLiquidationMessage, TickerMessage, TokenTransferMessage, TradeFillMessage,
    },
    types::{ProductSubscriptionMessage, SubaccountSubscriptionMessage},
    ws_client::{ClientError, WsClient},
};

pub struct Subscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> Subscriptions<'a> {
    pub async fn l2_book<F, Fut>(
        &self,
        symbols: Vec<String>,
        callback: F,
    ) -> Result<(), ClientError>
    where
        F: FnMut(L2BookMessage) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let payloads = symbols
            .iter()
            .map(|i| {
                ProductSubscriptionMessage {
                    msg_type: Channels::L2Book,
                    symbol: i.to_string(),
                }
                .into()
            })
            .collect::<Vec<Bytes>>();
        self.client
            .subscribe_channels(Channels::L2Book, payloads, callback)
            .await?;
        Ok(())
    }

    pub async fn order_fill<F, Fut>(
        &self,
        subaccount_ids: Vec<String>,
        callback: F,
    ) -> Result<(), ClientError>
    where
        F: FnMut(OrderFillMessage) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let payloads = subaccount_ids
            .iter()
            .map(|i| {
                SubaccountSubscriptionMessage {
                    msg_type: Channels::OrderFill,
                    subaccount_id: i.to_string(),
                }
                .into()
            })
            .collect::<Vec<Bytes>>();
        self.client
            .subscribe_channels(Channels::OrderFill, payloads, callback)
            .await?;
        Ok(())
    }

    pub async fn order_update<F, Fut>(
        &self,
        subaccount_ids: Vec<String>,
        callback: F,
    ) -> Result<(), ClientError>
    where
        F: FnMut(OrderUpdateMessage) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let payloads = subaccount_ids
            .iter()
            .map(|i| {
                SubaccountSubscriptionMessage {
                    msg_type: Channels::OrderUpdate,
                    subaccount_id: i.to_string(),
                }
                .into()
            })
            .collect::<Vec<Bytes>>();
        self.client
            .subscribe_channels(Channels::OrderUpdate, payloads, callback)
            .await?;
        Ok(())
    }

    pub async fn position_update<F, Fut>(
        &self,
        subaccount_ids: Vec<String>,
        callback: F,
    ) -> Result<(), ClientError>
    where
        F: FnMut(PositionUpdateMessage) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let payloads = subaccount_ids
            .iter()
            .map(|i| {
                SubaccountSubscriptionMessage {
                    msg_type: Channels::PositionUpdate,
                    subaccount_id: i.to_string(),
                }
                .into()
            })
            .collect::<Vec<Bytes>>();
        self.client
            .subscribe_channels(Channels::PositionUpdate, payloads, callback)
            .await?;
        Ok(())
    }

    pub async fn subaccount_liquidation<F, Fut>(
        &self,
        subaccount_ids: Vec<String>,
        callback: F,
    ) -> Result<(), ClientError>
    where
        F: FnMut(SubaccountLiquidationMessage) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let payloads = subaccount_ids
            .iter()
            .map(|i| {
                SubaccountSubscriptionMessage {
                    msg_type: Channels::SubaccountLiquidation,
                    subaccount_id: i.to_string(),
                }
                .into()
            })
            .collect::<Vec<Bytes>>();
        self.client
            .subscribe_channels(Channels::SubaccountLiquidation, payloads, callback)
            .await?;
        Ok(())
    }

    pub async fn ticker<F, Fut>(&self, symbols: Vec<String>, callback: F) -> Result<(), ClientError>
    where
        F: FnMut(TickerMessage) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let payloads = symbols
            .iter()
            .map(|i| {
                ProductSubscriptionMessage {
                    msg_type: Channels::Ticker,
                    symbol: i.to_string(),
                }
                .into()
            })
            .collect::<Vec<Bytes>>();
        self.client
            .subscribe_channels(Channels::Ticker, payloads, callback)
            .await?;
        Ok(())
    }

    pub async fn token_transfer<F, Fut>(
        &self,
        subaccount_ids: Vec<String>,
        callback: F,
    ) -> Result<(), ClientError>
    where
        F: FnMut(TokenTransferMessage) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let payloads = subaccount_ids
            .iter()
            .map(|i| {
                SubaccountSubscriptionMessage {
                    msg_type: Channels::TokenTransfer,
                    subaccount_id: i.to_string(),
                }
                .into()
            })
            .collect::<Vec<Bytes>>();
        self.client
            .subscribe_channels(Channels::TokenTransfer, payloads, callback)
            .await?;
        Ok(())
    }

    pub async fn trade_fill<F, Fut>(
        &self,
        symbols: Vec<String>,
        callback: F,
    ) -> Result<(), ClientError>
    where
        F: FnMut(TradeFillMessage) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let payloads = symbols
            .iter()
            .map(|i| {
                ProductSubscriptionMessage {
                    msg_type: Channels::TradeFill,
                    symbol: i.to_string(),
                }
                .into()
            })
            .collect::<Vec<Bytes>>();
        self.client
            .subscribe_channels(Channels::TradeFill, payloads, callback)
            .await?;
        Ok(())
    }
}
