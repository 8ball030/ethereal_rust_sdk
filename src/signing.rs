use alloy_dyn_abi::DynSolValue;
use alloy_primitives::{hex, keccak256, Address, Signature, B256, U256};
use alloy_signer::SignerSync;
use alloy_signer_local::PrivateKeySigner;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::SubaccountDto;
use crate::{domain_config::DOMAINS, enums::Environment};

use anyhow::{anyhow, Result};

/// Scales a decimal onto the wire's unsigned e9 fixed point. A negative value, or one whose
/// scaled form does not fit, is an error: the only two ways `to_u128` can fail, and this
/// function used to panic on both inside a `Result`.
pub fn to_scaled_e9(value: Decimal) -> Result<u128> {
    if value.is_sign_negative() && !value.is_zero() {
        return Err(anyhow!(
            "cannot scale {value} to e9: the wire format is unsigned"
        ));
    }
    let scaled = value
        .checked_mul(dec!(1_000_000_000))
        .ok_or_else(|| anyhow!("{value} * 1e9 does not fit in a Decimal"))?;
    scaled
        .to_u128()
        .ok_or_else(|| anyhow!("{scaled} does not fit in u128"))
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
    keccak256(
        DynSolValue::Tuple(vec![
            DynSolValue::FixedBytes(domain_type_hash, 32),
            DynSolValue::FixedBytes(keccak256(domain_config.name), 32),
            DynSolValue::FixedBytes(keccak256(domain_config.version), 32),
            DynSolValue::Uint(U256::from(domain_config.chain_id), 256),
            DynSolValue::Address(domain_config.verifying_contract.parse().unwrap()),
        ])
        .abi_encode_params(),
    )
    .0
}

/// Decodes a 32-byte hex string (with or without `0x`). Anything other than exactly 32 bytes is
/// an error: this value is a field the signature commits to, and silently padding a short value
/// or truncating a long one produced a valid-looking signature over the wrong bytes.
pub fn hex_to_bytes32(s: &str) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    let raw = s.trim_start_matches("0x");
    let decoded = hex::decode(raw)?;
    let out = <[u8; 32]>::try_from(decoded.as_slice()).map_err(|_| {
        format!(
            "expected 32 bytes of hex, got {} byte(s): {s:?}",
            decoded.len()
        )
    })?;
    Ok(out)
}
pub fn make_full_hash(domain_hash: &[u8; 32], message_hash: &[u8; 32]) -> [u8; 32] {
    let mut full_hash = Vec::with_capacity(66);
    full_hash.push(0x19);
    full_hash.push(0x01);
    full_hash.extend_from_slice(domain_hash);
    full_hash.extend_from_slice(message_hash);
    keccak256(&full_hash).0
}

#[derive(Debug, thiserror::Error)]
pub enum Eip712Error {
    #[error(transparent)]
    Signer(#[from] alloy_signer::Error),
}

// We define a base type
pub trait Eip712 {
    fn type_hash() -> Result<[u8; 32], Eip712Error>;
    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error>;
    fn sign(&self, env: Environment, wallet: &PrivateKeySigner) -> Result<Signature, Eip712Error> {
        let full_hash = self.encode_eip712(env)?;
        Ok(wallet.sign_hash_sync(&B256::from(full_hash))?)
    }
    fn encode_eip712(&self, env: Environment) -> Result<[u8; 32], Eip712Error> {
        let domain_separator = get_domain_separator(env);
        let full_hash = make_full_hash(&domain_separator, &self.struct_hash()?);
        Ok(full_hash)
    }
}

#[derive(Clone)]
pub struct EipSigningFields {
    pub sender: Address,
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
    wallet: &'a PrivateKeySigner,
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
    pub fn new(wallet: &'a PrivateKeySigner, subaccount: &'a SubaccountDto) -> Self {
        SigningContext {
            wallet,
            subaccount,
            nonce: get_nonce(),
            signed_at: get_now() as u64,
        }
    }
}
