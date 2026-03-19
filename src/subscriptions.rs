use std::future::Future;

use bytes::Bytes;

use crate::{
    channels::Channels,
    models::{L2BookMessage, OrderFillMessage, OrderUpdateMessage, TickerMessage},
    types::{ProductSubscriptionMessage, SubaccountSubscriptionMessage},
    ws_client::{ClientError, WsClient},
};

pub struct Subscriptions<'a> {
    pub client: &'a WsClient,
}
impl<'a> Subscriptions<'a> {
    pub async fn ticker<F, Fut>(&self, tickers: Vec<String>, callback: F) -> Result<(), ClientError>
    where
        F: FnMut(TickerMessage) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let payloads = tickers
            .iter()
            .map(|ticker| {
                ProductSubscriptionMessage {
                    msg_type: Channels::Ticker,
                    symbol: ticker.to_string(),
                }
                .into()
            })
            .collect::<Vec<Bytes>>();
        self.client
            .subscribe_channels(Channels::Ticker, payloads, callback)
            .await?;
        Ok(())
    }
    pub async fn l2_book<F, Fut>(
        &self,
        tickers: Vec<String>,
        callback: F,
    ) -> Result<(), ClientError>
    where
        F: FnMut(L2BookMessage) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let payloads = tickers
            .iter()
            .map(|ticker| {
                ProductSubscriptionMessage {
                    msg_type: Channels::L2Book,
                    symbol: ticker.to_string(),
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
        subaccounts: Vec<String>,
        callback: F,
    ) -> Result<(), ClientError>
    where
        F: FnMut(OrderFillMessage) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let payloads = subaccounts
            .iter()
            .map(|subaccount| {
                SubaccountSubscriptionMessage {
                    msg_type: Channels::OrderFill,
                    subaccount_id: subaccount.to_string(),
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
        subaccounts: Vec<String>,
        callback: F,
    ) -> Result<(), ClientError>
    where
        F: FnMut(OrderUpdateMessage) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let payloads = subaccounts
            .iter()
            .map(|subaccount| {
                SubaccountSubscriptionMessage {
                    msg_type: Channels::OrderUpdate,
                    subaccount_id: subaccount.to_string(),
                }
                .into()
            })
            .collect::<Vec<Bytes>>();
        self.client
            .subscribe_channels(Channels::OrderUpdate, payloads, callback)
            .await?;
        Ok(())
    }
}
