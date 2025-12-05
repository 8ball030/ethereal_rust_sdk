use ethers::signers::LocalWallet;
use ethers::types::transaction::eip712::Eip712Error;
use ethers::types::{Address, Signature};
use std::time::{SystemTime, UNIX_EPOCH};

use ethers::utils::hex;
use ethers::{types::U256, utils::keccak256};
use serde::{Deserialize, Serialize};

use crate::{domain_config::DOMAINS, enums::Environment, domain_config::DomainConfig};

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

pub fn get_now() -> i64 {
    // Get the current s timestamp to use as signed_at

    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_epoch.as_secs() as i64
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

pub fn hex_to_bytes32(s: &str) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    let raw = s.trim_start_matches("0x");
    let decoded = hex::decode(raw)?;

    let mut out = [0u8; 32];
    let n = decoded.len().min(32);
    out[..n].copy_from_slice(&decoded[..n]);

    Ok(out)
}
pub fn make_full_hash(domain_hash: &[u8; 32], message_hash: &[u8; 32]) -> [u8; 32] {
    let mut full_hash = Vec::with_capacity(66);
    full_hash.push(0x19);
    full_hash.push(0x01);
    full_hash.extend_from_slice(domain_hash);
    full_hash.extend_from_slice(message_hash);
    keccak256(&full_hash)
}



// We define a base type
pub trait Eip712 {
    fn type_hash() -> Result<[u8; 32], Eip712Error>;
    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error>;
    fn sign(&self, env: Environment, wallet: &LocalWallet) -> Result<Signature, Eip712Error> {
        let full_hash = self.encode_eip712(env)?;
        let signature = wallet.sign_hash(full_hash.into());
        Ok(signature.unwrap())
    }
    fn encode_eip712(&self, env: Environment) -> Result<[u8; 32], Eip712Error> {
        let domain_separator = get_domain_separator(env);
        let full_hash = make_full_hash(&domain_separator, &self.struct_hash()?);
        Ok(full_hash)
    }
}


// Define your typed data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOrder {
    pub sender: Address,
    pub subaccount: [u8; 32],
    pub quantity: u128,
    pub price: u128,
    pub reduce_only: bool,
    pub side: u8,
    pub engine_type: u8,
    pub product_id: u32,
    pub nonce: u64,
    pub signed_at: u64,
}
impl Eip712 for TradeOrder {
    
    fn type_hash() -> Result<[u8; 32], Eip712Error> {
        Ok(ethers::utils::keccak256(
            "TradeOrder(address sender,bytes32 subaccount,uint128 quantity,uint128 price,bool reduceOnly,uint8 side,uint8 engineType,uint32 productId,uint64 nonce,uint64 signedAt)",
        ))
    }
    
    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(&ethers::abi::encode(&[
            ethers::abi::Token::Address(self.sender),
            ethers::abi::Token::FixedBytes(self.subaccount.to_vec()),
            ethers::abi::Token::Uint(U256::from(self.quantity)),
            ethers::abi::Token::Uint(U256::from(self.price)),
            ethers::abi::Token::Bool(self.reduce_only),
            ethers::abi::Token::Uint(U256::from(self.side)),
            ethers::abi::Token::Uint(U256::from(self.engine_type)),
            ethers::abi::Token::Uint(U256::from(self.product_id)),
            ethers::abi::Token::Uint(U256::from(self.nonce)),
            ethers::abi::Token::Uint(U256::from(self.signed_at)),
        ]));
        
        Ok(ethers::utils::keccak256(&encoded))
    }
}
