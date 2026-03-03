use ethers::types::Address;
use ethers::types::transaction::eip712::Eip712Error;

use ethers::types::U256;
use serde::{Deserialize, Serialize};

use crate::signing::Eip712;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkSigner {
    pub sender: Address,
    pub signer: Address,
    pub subaccount: [u8; 32],
    pub nonce: u64,
    pub signed_at: u64,
}
impl Eip712 for LinkSigner {
    fn type_hash() -> Result<[u8; 32], Eip712Error> {
        Ok(ethers::utils::keccak256(
            "LinkSigner(address sender,address signer,bytes32 subaccount,uint64 nonce,uint64 signedAt)",
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(&ethers::abi::encode(&[
            ethers::abi::Token::Address(self.sender),
            ethers::abi::Token::Address(self.signer),
            ethers::abi::Token::FixedBytes(self.subaccount.to_vec()),
            ethers::abi::Token::Uint(U256::from(self.nonce)),
            ethers::abi::Token::Uint(U256::from(self.signed_at)),
        ]));

        Ok(ethers::utils::keccak256(&encoded))
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateWithdraw {
    pub account: Address,
    pub subaccount: [u8; 32],
    pub token: Address,
    pub amount: u128,
    pub nonce: u64,
    pub signed_at: u64,
    pub destination_address: [u8; 32],
    pub destination_endpoint_id: u32,
}
impl Eip712 for InitiateWithdraw {
    fn type_hash() -> Result<[u8; 32], Eip712Error> {
        Ok(ethers::utils::keccak256(
            "InitiateWithdraw(address account,bytes32 subaccount,address token,uint256 amount,uint64 nonce,uint64 signedAt,bytes32 destinationAddress,uint32 destinationEndpointId)",
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(&ethers::abi::encode(&[
            ethers::abi::Token::Address(self.account),
            ethers::abi::Token::FixedBytes(self.subaccount.to_vec()),
            ethers::abi::Token::Address(self.token),
            ethers::abi::Token::Uint(self.amount.into()),
            ethers::abi::Token::Uint(U256::from(self.nonce)),
            ethers::abi::Token::Uint(U256::from(self.signed_at)),
            ethers::abi::Token::FixedBytes(self.destination_address.to_vec()),
            ethers::abi::Token::Uint(U256::from(self.destination_endpoint_id)),
        ]));

        Ok(ethers::utils::keccak256(&encoded))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeLinkedSigner {
    pub sender: Address,
    pub signer: Address,
    pub subaccount: [u8; 32],
    pub nonce: u64,
    pub signed_at: u64,
}
impl Eip712 for RevokeLinkedSigner {
    fn type_hash() -> Result<[u8; 32], Eip712Error> {
        Ok(ethers::utils::keccak256(
            "RevokeLinkedSigner(address sender,address signer,bytes32 subaccount,uint64 nonce,uint64 signedAt)",
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(&ethers::abi::encode(&[
            ethers::abi::Token::Address(self.sender),
            ethers::abi::Token::Address(self.signer),
            ethers::abi::Token::FixedBytes(self.subaccount.to_vec()),
            ethers::abi::Token::Uint(U256::from(self.nonce)),
            ethers::abi::Token::Uint(U256::from(self.signed_at)),
        ]));

        Ok(ethers::utils::keccak256(&encoded))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EIP712Auth {
    pub sender: Address,
    pub intent: u8,
    pub signed_at: u64,
}
impl Eip712 for EIP712Auth {
    fn type_hash() -> Result<[u8; 32], Eip712Error> {
        Ok(ethers::utils::keccak256(
            "EIP712Auth(address sender,uint8 intent,uint64 signedAt)",
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(&ethers::abi::encode(&[
            ethers::abi::Token::Address(self.sender),
            ethers::abi::Token::Uint(U256::from(self.intent)),
            ethers::abi::Token::Uint(U256::from(self.signed_at)),
        ]));

        Ok(ethers::utils::keccak256(&encoded))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrder {
    pub sender: Address,
    pub subaccount: [u8; 32],
    pub nonce: u64,
}
impl Eip712 for CancelOrder {
    fn type_hash() -> Result<[u8; 32], Eip712Error> {
        Ok(ethers::utils::keccak256(
            "CancelOrder(address sender,bytes32 subaccount,uint64 nonce)",
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(&ethers::abi::encode(&[
            ethers::abi::Token::Address(self.sender),
            ethers::abi::Token::FixedBytes(self.subaccount.to_vec()),
            ethers::abi::Token::Uint(U256::from(self.nonce)),
        ]));

        Ok(ethers::utils::keccak256(&encoded))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshLinkedSigner {
    pub sender: Address,
    pub signer: Address,
    pub nonce: u64,
    pub signed_at: u64,
}
impl Eip712 for RefreshLinkedSigner {
    fn type_hash() -> Result<[u8; 32], Eip712Error> {
        Ok(ethers::utils::keccak256(
            "RefreshLinkedSigner(address sender,address signer,uint64 nonce,uint64 signedAt)",
        ))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(&ethers::abi::encode(&[
            ethers::abi::Token::Address(self.sender),
            ethers::abi::Token::Address(self.signer),
            ethers::abi::Token::Uint(U256::from(self.nonce)),
            ethers::abi::Token::Uint(U256::from(self.signed_at)),
        ]));

        Ok(ethers::utils::keccak256(&encoded))
    }
}
