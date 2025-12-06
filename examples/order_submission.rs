mod common;
use ethereal_rust_sdk::apis::order_api::OrderControllerSubmitParams;
use ethereal_rust_sdk::apis::product_api::ProductControllerListParams;
use ethereal_rust_sdk::apis::subaccount_api::SubaccountControllerListByAccountParams;
use ethereal_rust_sdk::enums::Environment;
use ethereal_rust_sdk::models::{
    OrderSide, SubmitOrderDto, SubmitOrderDtoData, SubmitOrderLimitDtoData,
};
use ethereal_rust_sdk::signing::Eip712;
use ethereal_rust_sdk::signing::{get_nonce, get_now, hex_to_bytes32, to_scaled_e9, TradeOrder};
use ethers::signers::{LocalWallet, Signer};

use ethers::utils::hex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create wallet
    let wallet = "0bb5d63b84421e1268dda020818ae30cf26e7f10e321fb820a8aa69216dea92a"
        .parse::<LocalWallet>()?;

    println!("Wallet address: {:?}", wallet.address());
    println!("Creating order...");

    let env = Environment::Testnet;

    let (http_client, _) = common::create_test_clients()?;

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
    let now = get_now();

    let human_quantity = 0.001;
    let human_price = 80000.0;

    let message = TradeOrder {
        sender: sender_address.parse()?,
        subaccount: hex_to_bytes32(&subaccount.name)?,
        quantity: to_scaled_e9(human_quantity),
        price: to_scaled_e9(human_price),
        reduce_only: false,
        side: OrderSide::BUY as u8,
        engine_type: btc_product.engine_type.to_string().parse()?,
        product_id: btc_product.onchain_id.to_string().parse()?,
        nonce,
        signed_at: now as u64,
    };

    println!("Message: {}", serde_json::to_string_pretty(&message)?);
    // Encode and sign
    let hash = message.struct_hash()?;
    println!("EIP712 Hash as hex: 0x{}", hex::encode(hash));

    let signature = message.sign(env, &wallet)?;
    println!("Signature: 0x{signature}");
    let dto = SubmitOrderDto {
        data: Box::new(SubmitOrderDtoData::SubmitOrderLimitDtoData(Box::new(
            SubmitOrderLimitDtoData {
                subaccount: subaccount.name,
                sender: sender_address.to_string(),
                nonce: nonce.to_string(),
                quantity: human_quantity.to_string(),
                side: OrderSide::BUY,
                onchain_id: btc_product.onchain_id,
                engine_type: btc_product.engine_type,
                reduce_only: Some(false),
                signed_at: now,
                price: human_price.to_string(),
                ..Default::default()
            },
        ))),
        signature: "0x".to_string() + &hex::encode(signature.to_vec()),
    };

    // we json dump the dto to see what we are sending
    let dto_json = serde_json::to_string_pretty(&dto)?;
    println!("Submitting order with DTO: {dto_json}");

    let result = http_client.order().submit(OrderControllerSubmitParams {
        submit_order_dto: dto,
    });
    println!("Order submission result: {result:?}");
    Ok(())
}
