use ethereal_rust_sdk::apis::order_api::OrderControllerSubmitParams;
use ethereal_rust_sdk::apis::product_api::ProductControllerListParams;
use ethereal_rust_sdk::apis::subaccount_api::SubaccountControllerListByAccountParams;
use ethereal_rust_sdk::enums::Environment;
use ethereal_rust_sdk::models::{
    OrderSide, SubmitOrderDto, SubmitOrderDtoData, SubmitOrderMarketDtoData,
};
use ethereal_rust_sdk::signing::{
    encode_eip712_message, get_nonce, get_now, to_scaled_e9, TradeOrder,
};
use ethereal_rust_sdk::sync_client::client::HttpClient;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::U128;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create wallet
    let wallet = "0bb5d63b84421e1268dda020818ae30cf26e7f10e321fb820a8aa69216dea92a"
        .parse::<LocalWallet>()?;

    println!("Wallet address: {:?}", wallet.address());
    println!("Creating order...");

    let env = Environment::Testnet;

    let http_client = HttpClient::new(env.clone());

    let btc_product = http_client
        .product()
        .list(ProductControllerListParams {
            ticker: Some("BTC".to_string()),
            ..Default::default()
        })?
        .data
        .first()
        .ok_or_else(|| anyhow::anyhow!("BTC-USD product not found"))?
        .clone();

    println!("BTC Product: {btc_product:?}");

    let sender_address = format!("{:?}", wallet.address());
    println!("Using wallet address: {sender_address:?}\n");

    let subaccount = http_client
        .subaccount()
        .list_by_account(SubaccountControllerListByAccountParams {
            sender: sender_address.to_string(),
            ..Default::default()
        })?
        .data
        .first()
        .ok_or_else(|| anyhow::anyhow!("No subaccounts found for account"))?
        .clone();
    println!("Using subaccount: {subaccount:?}");
    // Create message
    let nonce = get_nonce(); // implement get_nonce to fetch or generate a nonce
    let now = get_now(); // implement get_signed_at to get current timestamp

    let human_quantity = 0.001;
    let human_price = 80000.0;

    let message = TradeOrder {
        sender: sender_address.parse()?,
        subaccount: subaccount.name.parse()?,
        quantity: U128::from(to_scaled_e9(human_quantity)),
        price: U128::from(to_scaled_e9(human_price)),
        reduce_only: false,
        side: OrderSide::BUY as u8,
        engine_type: btc_product.engine_type.to_string().parse()?,
        product_id: btc_product.onchain_id.to_string().parse()?,
        nonce,
        signed_at: now,
    };

    println!("Message: {message:?}");
    // Encode and sign
    let hash = encode_eip712_message(&message);
    println!("EIP712 Hash: {hash}");

    let signature = wallet.sign_hash(hash)?;
    println!("Signature: 0x{signature}");
    let dto = SubmitOrderDto {
        data: Box::new(SubmitOrderDtoData::SubmitOrderMarketDtoData(Box::new(
            SubmitOrderMarketDtoData {
                subaccount: subaccount.name.to_string(),
                sender: sender_address.to_string(),
                nonce: nonce.to_string(),
                quantity: human_quantity.to_string(),
                onchain_id: btc_product.onchain_id,
                engine_type: btc_product.engine_type,
                reduce_only: Some(false),
                close: Some(false),
                side: OrderSide::BUY,
                signed_at: now as f64,
                ..Default::default()
            },
        ))),
        signature: format!("0x{signature}"),
    };

    let result = http_client.order().submit(OrderControllerSubmitParams {
        submit_order_dto: dto,
    });
    println!("Order submission result: {result:?}");
    Ok(())
}
