use alloy_dyn_abi::DynSolValue;
use alloy_primitives::{keccak256, Address, B256, U256};
use serde::{Deserialize, Serialize};

use crate::signing::{Eip712, Eip712Error};

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
        Ok(keccak256(
            "LinkSigner(address sender,address signer,bytes32 subaccount,uint64 nonce,uint64 signedAt)"
        ).0)
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(
            &DynSolValue::Tuple(vec![
                DynSolValue::Address(self.sender),
                DynSolValue::Address(self.signer),
                DynSolValue::FixedBytes(B256::from(self.subaccount), 32),
                DynSolValue::Uint(U256::from(self.nonce), 64),
                DynSolValue::Uint(U256::from(self.signed_at), 64),
            ])
            .abi_encode_params(),
        );

        Ok(keccak256(&encoded).0)
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
        Ok(keccak256(
            "TradeOrder(address sender,bytes32 subaccount,uint128 quantity,uint128 price,bool reduceOnly,uint8 side,uint8 engineType,uint32 productId,uint64 nonce,uint64 signedAt)"
        ).0)
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(
            &DynSolValue::Tuple(vec![
                DynSolValue::Address(self.sender),
                DynSolValue::FixedBytes(B256::from(self.subaccount), 32),
                DynSolValue::Uint(U256::from(self.quantity), 128),
                DynSolValue::Uint(U256::from(self.price), 128),
                DynSolValue::Bool(self.reduce_only),
                DynSolValue::Uint(U256::from(self.side), 8),
                DynSolValue::Uint(U256::from(self.engine_type), 8),
                DynSolValue::Uint(U256::from(self.product_id), 32),
                DynSolValue::Uint(U256::from(self.nonce), 64),
                DynSolValue::Uint(U256::from(self.signed_at), 64),
            ])
            .abi_encode_params(),
        );

        Ok(keccak256(&encoded).0)
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
        Ok(keccak256(
            "InitiateWithdraw(address account,bytes32 subaccount,address token,uint256 amount,uint64 nonce,uint64 signedAt,bytes32 destinationAddress,uint32 destinationEndpointId)"
        ).0)
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(
            &DynSolValue::Tuple(vec![
                DynSolValue::Address(self.account),
                DynSolValue::FixedBytes(B256::from(self.subaccount), 32),
                DynSolValue::Address(self.token),
                DynSolValue::Uint(U256::from(self.amount), 256),
                DynSolValue::Uint(U256::from(self.nonce), 64),
                DynSolValue::Uint(U256::from(self.signed_at), 64),
                DynSolValue::FixedBytes(B256::from(self.destination_address), 32),
                DynSolValue::Uint(U256::from(self.destination_endpoint_id), 32),
            ])
            .abi_encode_params(),
        );

        Ok(keccak256(&encoded).0)
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
        Ok(keccak256(
            "RevokeLinkedSigner(address sender,address signer,bytes32 subaccount,uint64 nonce,uint64 signedAt)"
        ).0)
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(
            &DynSolValue::Tuple(vec![
                DynSolValue::Address(self.sender),
                DynSolValue::Address(self.signer),
                DynSolValue::FixedBytes(B256::from(self.subaccount), 32),
                DynSolValue::Uint(U256::from(self.nonce), 64),
                DynSolValue::Uint(U256::from(self.signed_at), 64),
            ])
            .abi_encode_params(),
        );

        Ok(keccak256(&encoded).0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertToken {
    pub sender: Address,
    pub subaccount: [u8; 32],
    pub from_token: Address,
    pub to_token: Address,
    pub amount: u128,
    pub nonce: u64,
    pub signed_at: u64,
}
impl Eip712 for ConvertToken {
    fn type_hash() -> Result<[u8; 32], Eip712Error> {
        Ok(keccak256(
            "ConvertToken(address sender,bytes32 subaccount,address fromToken,address toToken,uint128 amount,uint64 nonce,uint64 signedAt)"
        ).0)
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(
            &DynSolValue::Tuple(vec![
                DynSolValue::Address(self.sender),
                DynSolValue::FixedBytes(B256::from(self.subaccount), 32),
                DynSolValue::Address(self.from_token),
                DynSolValue::Address(self.to_token),
                DynSolValue::Uint(U256::from(self.amount), 128),
                DynSolValue::Uint(U256::from(self.nonce), 64),
                DynSolValue::Uint(U256::from(self.signed_at), 64),
            ])
            .abi_encode_params(),
        );

        Ok(keccak256(&encoded).0)
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
        Ok(keccak256("EIP712Auth(address sender,uint8 intent,uint64 signedAt)").0)
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(
            &DynSolValue::Tuple(vec![
                DynSolValue::Address(self.sender),
                DynSolValue::Uint(U256::from(self.intent), 8),
                DynSolValue::Uint(U256::from(self.signed_at), 64),
            ])
            .abi_encode_params(),
        );

        Ok(keccak256(&encoded).0)
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
        Ok(keccak256("CancelOrder(address sender,bytes32 subaccount,uint64 nonce)").0)
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(
            &DynSolValue::Tuple(vec![
                DynSolValue::Address(self.sender),
                DynSolValue::FixedBytes(B256::from(self.subaccount), 32),
                DynSolValue::Uint(U256::from(self.nonce), 64),
            ])
            .abi_encode_params(),
        );

        Ok(keccak256(&encoded).0)
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
        Ok(keccak256(
            "RefreshLinkedSigner(address sender,address signer,uint64 nonce,uint64 signedAt)",
        )
        .0)
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(
            &DynSolValue::Tuple(vec![
                DynSolValue::Address(self.sender),
                DynSolValue::Address(self.signer),
                DynSolValue::Uint(U256::from(self.nonce), 64),
                DynSolValue::Uint(U256::from(self.signed_at), 64),
            ])
            .abi_encode_params(),
        );

        Ok(keccak256(&encoded).0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendLinkedSigner {
    pub sender: Address,
    pub nonce: u64,
    pub signed_at: u64,
}
impl Eip712 for ExtendLinkedSigner {
    fn type_hash() -> Result<[u8; 32], Eip712Error> {
        Ok(keccak256("ExtendLinkedSigner(address sender,uint64 nonce,uint64 signedAt)").0)
    }

    fn struct_hash(&self) -> Result<[u8; 32], Eip712Error> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()?);
        encoded.extend_from_slice(
            &DynSolValue::Tuple(vec![
                DynSolValue::Address(self.sender),
                DynSolValue::Uint(U256::from(self.nonce), 64),
                DynSolValue::Uint(U256::from(self.signed_at), 64),
            ])
            .abi_encode_params(),
        );

        Ok(keccak256(&encoded).0)
    }
}
