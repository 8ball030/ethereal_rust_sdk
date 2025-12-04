use alloy::hex;
use alloy::primitives::{Address, U256, address, FixedBytes};
use alloy::signers::k256::ecdsa::SigningKey;
use alloy::signers::{local::PrivateKeySigner, Signer};
use alloy::sol_types::{sol, eip712_domain, Eip712Domain};
use alloy_signer::k256::sha2::digest::typenum::uint;

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


// {
//   "name": "Ethereal",
//   "version": "1",
//   "chainId": 5064014,
//   "verifyingContract": "0xB3cDC82035C495c484C9fF11eD5f3Ff6d342e3cc"
// }

const DOMAIN: Eip712Domain = eip712_domain! {
    name: "Ethereal",
    version: "1",
    chain_id: 5064014u64,
    verifying_contract: address!(
        "0xB3cDC82035C495c484C9fF11eD5f3Ff6d342e3cc"
    ),
};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1) Local signer from private key
    let pk_hex = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcd";
    let signer: PrivateKeySigner = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;
 

    // 3) Build the order struct.
    // Here you’d already have applied your 1e9 scaling etc on quantity/price.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let order = TradeOrder {
        sender: signer.address(),
        subaccount:FixedBytes([0u8; 32]),
        quantity: 1_000_000_000, // 1.0 quantity with 1e9 scaling
        price: 25_000_000_000, // 25.0 price with 1e9 scaling
        reduceOnly: false,
        side: 0u8,          // BUY
        engineType: 0u8,   // SPOT
        productId: 42u32,  // example product ID
        nonce: 1u64,       // should be unique per order
        signedAt: now,     // current timestamp
    };

    // 4) Sign typed data. This hashes according to EIP-712 and signs with secp256k1. :contentReference[oaicite:2]{index=2}
    let sig = signer.sign_typed_data(&order, &DOMAIN).await?;


    println!("Address:        {:?}", signer.address());
    println!("Signature bytes: 0x{}", hex::encode(sig.as_bytes()));
    println!("v: {}, r: 0x{}, s: 0x{}",
        sig.v(),
        hex::encode(sig.r().to_be_bytes::<32>()),
        hex::encode(sig.s().to_be_bytes::<32>()),
    );
    Ok(())

}
