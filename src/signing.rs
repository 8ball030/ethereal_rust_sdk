use ethers::types::transaction::eip712::Eip712Error;
use ethers::types::{Signature, H160};
use ethers::{
    signers::{LocalWallet, Signer},
    utils::hex,
};
use std::time::{SystemTime, UNIX_EPOCH};

use ethers::{types::U256, utils::keccak256};

use crate::models::SubaccountDto;
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

#[derive(Clone)]
pub struct EipSigningFields {
    pub sender: H160,
    pub subaccount: [u8; 32],
    pub nonce: u64,
    pub signed_at: u64,
}

#[derive(Clone)]
pub struct DtoSigningFields {
    pub sender: String,
    pub subaccount: String,
    pub nonce: String,
    pub signed_at: i64,
}
pub struct SigningContext<'a> {
    wallet: &'a LocalWallet,
    subaccount: &'a SubaccountDto,
    pub nonce: u64,
    pub signed_at: u64,
}
impl<'a> SigningContext<'a> {
    pub fn eip_signing_fields(&self) -> EipSigningFields {
        EipSigningFields {
            sender: self.wallet.address(),
            subaccount: hex_to_bytes32(&self.subaccount.name).unwrap(),
            nonce: self.nonce,
            signed_at: self.signed_at,
        }
    }
    pub fn dto_signing_fields(&self) -> DtoSigningFields {
        DtoSigningFields {
            sender: format!("{:?}", self.wallet.address()),
            subaccount: self.subaccount.name.clone(),
            nonce: self.nonce.to_string(),
            signed_at: self.signed_at as i64,
        }
    }
    pub fn new(wallet: &'a LocalWallet, subaccount: &'a SubaccountDto) -> Self {
        SigningContext {
            wallet,
            subaccount,
            nonce: get_nonce(),
            signed_at: get_now() as u64,
        }
    }
}
