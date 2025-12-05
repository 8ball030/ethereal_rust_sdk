use std::time::{SystemTime, UNIX_EPOCH};

use ethers::{
    types::{Address, H256, U128, U256},
    utils::keccak256,
};
use serde::{Deserialize, Serialize};

use crate::{domain_config::DOMAINS, enums::Environment};

pub fn to_scaled_e9(value: f64) -> u128 {
    (value * 1e9) as u128
}

pub fn get_nonce() -> u64 {
    // Get the current ns timestamp to use as a nonce

    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_epoch.as_nanos() as u64
}

pub fn get_now() -> u64 {
    // Get the current s timestamp to use as signed_at

    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_epoch.as_secs()
}

pub fn get_domain_separator(env: Environment) -> [u8; 32] {
    let domain_config = DOMAINS.get(env);
    let domain_type_hash = keccak256(
        "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)",
    );
    keccak256(ethers::abi::encode(&[
        ethers::abi::Token::Uint(U256::from(domain_type_hash)),
        ethers::abi::Token::Uint(U256::from(keccak256(domain_config.name))),
        ethers::abi::Token::Uint(U256::from(keccak256(domain_config.version))),
        ethers::abi::Token::Uint(U256::from(domain_config.chain_id)),
        ethers::abi::Token::Address(domain_config.verifying_contract.parse().unwrap()),
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOrder {
    pub sender: Address,
    pub subaccount: H256,
    pub quantity: U128,
    pub price: U128,
    pub reduce_only: bool,
    pub side: u8,
    pub engine_type: u8,
    pub product_id: u32,
    pub nonce: u64,
    pub signed_at: u64,
}

pub fn encode_eip712_message(message: &TradeOrder) -> H256 {
    // EIP712 Domain type hash
    let env = Environment::Testnet;
    // Domain Separator
    let domain_separator = get_domain_separator(env);
    // Message type hash
    let message_type_hash = keccak256(
        "TradeOrder(address sender,bytes32 subaccount,uint128 quantity,uint128 price,bool reduceOnly,uint8 side,uint8 engineType,uint32 productId,uint64 nonce,uint64 signedAt)",
    );

    // Struct hash
    let struct_hash = keccak256(ethers::abi::encode(&[
        ethers::abi::Token::FixedBytes(message_type_hash.to_vec()),
        ethers::abi::Token::Address(message.sender),
        ethers::abi::Token::FixedBytes(message.subaccount.as_bytes().to_vec()), // <-- FIX HERE
        ethers::abi::Token::Uint(U256::from(message.quantity)),
        ethers::abi::Token::Uint(U256::from(message.price)),
        ethers::abi::Token::Bool(message.reduce_only),
        ethers::abi::Token::Uint(U256::from(message.side)),
        ethers::abi::Token::Uint(U256::from(message.engine_type)),
        ethers::abi::Token::Uint(U256::from(message.product_id)),
        ethers::abi::Token::Uint(U256::from(message.nonce)),
        ethers::abi::Token::Uint(U256::from(message.signed_at)),
    ]));

    // Final EIP712 hash: keccak256("\x19\x01" ‖ domainSeparator ‖ structHash)
    let mut encoded = vec![0x19, 0x01];
    encoded.extend_from_slice(&domain_separator);
    encoded.extend_from_slice(&struct_hash);

    H256::from_slice(&keccak256(encoded))
}
