use alloy::hex;
use alloy::primitives::{address, FixedBytes};
use alloy::signers::{local::PrivateKeySigner, SignerSync};
use alloy::sol_types::{eip712_domain, sol, Eip712Domain};
use ethereal_rust_sdk::apis::order_api::OrderControllerSubmitParams;
use ethereal_rust_sdk::apis::product_api::ProductControllerListParams;
use ethereal_rust_sdk::apis::subaccount_api::SubaccountControllerListByAccountParams;
use ethereal_rust_sdk::enums::Environment;
use ethereal_rust_sdk::models::{
    OrderSide, SubmitOrderDto, SubmitOrderDtoData, SubmitOrderMarketDtoData,
};
use ethereal_rust_sdk::sync_client::client::HttpClient;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

// "address sender,bytes32 subaccount,uint128 quantity,uint128 price,bool reduceOnly,uint8 side,uint8 engineType,uint32 productId,uint64 nonce,uint64 signedAt"

// Define the EIP-712 struct in Solidity syntax.
sol! {
    #[derive(Debug)]
    struct TradeOrder {
        address sender;
        bytes32 subaccount;
        uint128 quantity;
        uint128 price;
        bool reduceOnly;
        uint8 side;
        uint8 engineType;
        uint32 productId;
        uint64 nonce;
        uint64 signedAt;
    }
}

const DOMAIN: Eip712Domain = eip712_domain! {
    name: "Ethereal",
    version: "1",
    chain_id: 5064014u64,
    verifying_contract: address!(
        "0xB3cDC82035C495c484C9fF11eD5f3Ff6d342e3cc"
    ),
};

fn to_scaled_e9(value: f64) -> u128 {
    (value * 1e9) as u128
}

pub fn uuid_to_bytes32(uuid_str: &Uuid) -> u32 {
    // Parse UUID
    // Extract raw 16 bytes
    let raw = uuid_str.as_bytes();

    // Convert to hex (32 hex chars)
    let hex16 = hex::encode(raw);

    // Pad to 32 bytes = 64 hex characters (left pad with '0')
    let padded = format!("{hex16:0>64}");
    // Take first 8 characters (4 bytes)
    let first8 = &padded[0..8];
    // Convert to u32
    u32::from_str_radix(first8, 16).expect("Failed to convert to u32")
}

pub fn get_nonce() -> u64 {
    // Get the current ns timestamp to use as a nonce

    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_epoch.as_nanos() as u64
}

fn main() -> anyhow::Result<()> {
    // 1) Local signer from private key
    // We have 10 signers available in the Ethereal local testnet.

    dotenvy::dotenv().ok(); // load .env at runtime

    let signer: PrivateKeySigner = {
        let pk_hex = std::env::var("ETHEREAL_SIGNER_PRIVATE_KEY")
            .expect("ETHEREAL_SIGNER_PRIVATE_KEY must be set in .env");
        pk_hex.parse()?
    };
    println!("Using signer address: {:?}", signer.address());
    println!("Creating order...");

    let env = Environment::Testnet;

    println!("Available products:");

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

    let subaccount = http_client
        .subaccount()
        .list_by_account(SubaccountControllerListByAccountParams {
            sender: signer.address().to_string(),
            ..Default::default()
        })?
        .data
        .first()
        .ok_or_else(|| anyhow::anyhow!("No subaccounts found for account"))?
        .clone();
    println!("Using subaccount: {subaccount:?}");

    let product_id_bytes32 = uuid_to_bytes32(&btc_product.id);

    let subaccount_id_bytes32: FixedBytes<32> = {
        let raw = subaccount.id.as_bytes();
        let hex16 = hex::encode(raw);
        let padded = format!("{hex16:0>64}");
        let bytes = hex::decode(padded).expect("Failed to decode hex");
        FixedBytes::<32>::from_slice(&bytes)
    };

    // 3) Build the order struct.
    // Here you’d already have applied your 1e9 scaling etc on quantity/price.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let price = to_scaled_e9(125000.0);
    let quantity = to_scaled_e9(0.001);
    let order = TradeOrder {
        sender: signer.address(),
        subaccount: subaccount_id_bytes32,
        reduceOnly: false,
        side: 0u8,                     // BUY
        engineType: 0u8,               // SPOT
        productId: product_id_bytes32, // example product ID
        nonce: get_nonce(),            // should be unique per order
        signedAt: now,                 // current timestamp
        quantity,
        price,
    };

    // 4) Sign typed data. This hashes according to EIP-712 and signs with secp256k1. :contentReference[oaicite:2]{index=2}
    let sig = signer.sign_typed_data_sync(&order, &DOMAIN)?;

    println!("Address:        {:?}", signer.address());
    println!("Signature bytes: 0x{}", hex::encode(sig.as_bytes()));
    println!(
        "v: {}, r: 0x{}, s: 0x{}",
        sig.v(),
        hex::encode(sig.r().to_be_bytes::<32>()),
        hex::encode(sig.s().to_be_bytes::<32>()),
    );

    let dto = SubmitOrderDto {
        data: Box::new(SubmitOrderDtoData::SubmitOrderMarketDtoData(Box::new(
            SubmitOrderMarketDtoData {
                subaccount: subaccount.name.to_string(),
                sender: signer.address().to_string(),
                nonce: order.nonce.to_string(),
                quantity: quantity.to_string(),
                onchain_id: btc_product.onchain_id,
                engine_type: btc_product.engine_type,
                reduce_only: Some(false),
                close: Some(false),
                side: OrderSide::BUY,
                signed_at: now as f64,
                ..Default::default()
            },
        ))),
        signature: format!("0x{}", hex::encode(sig.as_bytes())),
    };

    let result = http_client.order().submit(OrderControllerSubmitParams {
        submit_order_dto: dto,
    });
    println!("Order submission result: {result:?}");
    Ok(())
}
