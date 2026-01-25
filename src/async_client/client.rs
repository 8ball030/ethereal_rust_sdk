use crate::{
    apis::{
        configuration::Configuration,
        order_api::{
            OrderControllerCancelParams, OrderControllerListBySubaccountIdParams,
            OrderControllerSubmitParams,
        },
        product_api::ProductControllerListParams,
        subaccount_api::SubaccountControllerListByAccountParams,
    },
    async_client::{
        funding::FundingClient,
        linked_signer::LinkedSignerClient,
        maintenance::MaintenanceClient,
        order::OrderClient,
        points::PointsClient,
        position::PositionClient,
        product::{self, ProductClient},
        referral::ReferralClient,
        rpc::RpcClient,
        subaccount::SubaccountClient,
        time::TimeClient,
        token::TokenClient,
        whitelist::WhitelistClient,
    },
    enums::Environment,
    models::{
        CancelOrderDto, CancelOrderDtoData, CancelOrderResultDto, OrderStatus, SubaccountDto,
        SubmitOrderCreatedDto, SubmitOrderDto, SubmitOrderDtoData, SubmitOrderLimitDtoData,
    },
    signable_messages::{CancelOrder, TradeOrder},
    signing::{get_nonce, get_now, hex_to_bytes32, to_scaled_e9},
};

use crate::models::submit_order_limit_dto_data::TimeInForce;
use crate::signing::Eip712;
use ethers::{
    signers::{LocalWallet, Signer},
    utils::hex,
};
use uuid::Uuid;

fn get_server_url(environment: &Environment) -> &str {
    match environment {
        Environment::Mainnet => "https://api.ethereal.trade",
        Environment::Testnet => "https://api.etherealtest.net",
    }
}

pub struct HttpClient {
    env: Environment,
    config: Configuration,
    pub wallet: LocalWallet,
    pub address: String,
    pub subaccounts: Vec<SubaccountDto>,
    product_hashmap: std::collections::HashMap<String, crate::models::ProductDto>,
}

impl HttpClient {
    pub async fn new(env: Environment, private_key: &str) -> Self {
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
            .await
            .unwrap()
            .data;
        let product_hashmap = product::ProductClient { config: &config }
            .list(ProductControllerListParams {
                ..Default::default()
            })
            .await
            .unwrap()
            .data
            .into_iter()
            .map(|p| (p.display_ticker.clone(), p))
            .collect();

        Self {
            env,
            config,
            wallet,
            address,
            subaccounts,
            product_hashmap,
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

    #[allow(clippy::too_many_arguments)]
    pub async fn submit_order(
        &self,
        ticker: &str,
        quantity: f64,
        price: Option<f64>,
        side: crate::models::OrderSide,
        r#type: crate::models::OrderType,
        time_in_force: TimeInForce,
        post_only: bool,
        reduce_only: bool,
        expires_at: Option<i64>,
    ) -> Result<SubmitOrderCreatedDto, Box<dyn std::error::Error>> {
        println!(
            "Submitting order... of type {type:?} and side {side:?} for {quantity} {ticker} at {price:?} reduce only: {reduce_only} post only: {post_only} expires at: {expires_at:?}"
        );
        if !self.product_hashmap.contains_key(ticker) {
            return Err(format!("Ticker {ticker} not found").into());
        }
        let product_info = self.product_hashmap.get(ticker).unwrap();
        let nonce = get_nonce(); // implement get_nonce to fetch or generate a nonce
        let now = get_now();
        let message = TradeOrder {
            sender: self.address.parse()?,
            subaccount: hex_to_bytes32(&self.subaccounts[0].name)?,
            quantity: to_scaled_e9(quantity),
            price: to_scaled_e9(price.unwrap_or(0.0)),
            reduce_only,
            side: side as u8,
            engine_type: product_info.engine_type.to_string().parse()?,
            product_id: product_info.onchain_id.to_string().parse()?,
            nonce,
            signed_at: now as u64,
        };
        let signature = message.sign(self.env, &self.wallet)?;

        let dto = SubmitOrderDto {
            data: Box::new(SubmitOrderDtoData::SubmitOrderLimitDtoData(Box::new(
                SubmitOrderLimitDtoData {
                    subaccount: self.subaccounts[0].name.clone(),
                    sender: self.address.to_string(),
                    nonce: nonce.to_string(),
                    quantity: quantity.to_string(),
                    side,
                    onchain_id: product_info.onchain_id,
                    engine_type: product_info.engine_type,
                    reduce_only: Some(reduce_only),
                    signed_at: now,
                    price: price.expect("Price should be present").to_string(),
                    post_only,
                    expires_at,
                    time_in_force,
                    ..Default::default()
                },
            ))),
            signature: "0x".to_string() + &hex::encode(signature.to_vec()),
        };

        let result = self
            .order()
            .submit(OrderControllerSubmitParams {
                submit_order_dto: dto,
            })
            .await;
        match result {
            Ok(response) => Ok(response),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn cancel_orders(
        &self,
        order_ids: Vec<String>,
    ) -> Result<Vec<CancelOrderResultDto>, Box<dyn std::error::Error>> {
        println!("Cancelling order...");
        let subaccount = &self.subaccounts[0];
        let nonce = get_nonce(); // implement get_nonce to fetch or generate a nonce
        let message = CancelOrder {
            sender: self.address.clone().parse()?,
            subaccount: hex_to_bytes32(&subaccount.name.clone())?,
            nonce, // increment nonce for the cancel order
        };

        let signature = message.sign(self.env, &self.wallet)?;
        let ids: Vec<Uuid> = order_ids
            .iter()
            .map(|id| Uuid::parse_str(id).unwrap())
            .collect();
        let cancel_result = self
            .order()
            .cancel(OrderControllerCancelParams {
                cancel_order_dto: CancelOrderDto {
                    data: Box::new(CancelOrderDtoData {
                        subaccount: subaccount.name.clone(),
                        sender: self.address.to_string(),
                        nonce: nonce.to_string(),
                        order_ids: ids.into(),
                        ..Default::default()
                    }),
                    signature: "0x".to_string() + &hex::encode(signature.to_vec()),
                },
            })
            .await;
        Ok(cancel_result.unwrap().data)
    }
    pub async fn get_open_orders(
        &self,
    ) -> Result<Vec<crate::models::OrderDto>, Box<dyn std::error::Error>> {
        let orders = self
            .order()
            .list_by_subaccount_id(OrderControllerListBySubaccountIdParams {
                subaccount_id: self.subaccounts[0].id.clone().to_string(),
                ..Default::default()
            })
            .await?
            .data;
        let open_orders = orders
            .into_iter()
            .filter(|order| order.status == OrderStatus::New);
        Ok(open_orders.collect())
    }
}
