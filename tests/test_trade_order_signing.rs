use ethereal_rust_sdk::enums::Environment;
use ethereal_rust_sdk::signing::{TradeOrder, to_scaled_e9};
use ethereal_rust_sdk::signing::{
    get_domain_separator, hex_to_bytes32, make_full_hash, sign_eip712,
};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::transaction::eip712::Eip712;
use ethers::utils::hex;

fn get_test_order() -> TradeOrder {
    TradeOrder {
        sender: "0xdeadbeef00000000000000000000000000000000"
            .parse()
            .unwrap(),
        subaccount: hex_to_bytes32(
            "0x123456789abcde00000000000000000000000000000000000000000000000000",
        )
        .unwrap(),
        quantity: to_scaled_e9(1.0),
        price: to_scaled_e9(3000.0),
        reduce_only: false,
        side: 0,
        engine_type: 0,
        product_id: 2,
        nonce: 1764897077655477722,
        signed_at: 1764897077,
        environment: Environment::Testnet,
    }
}

#[test]
fn test_to_scaled_e9() {
    let value = 1.23456789;
    let scaled = to_scaled_e9(value);
    assert_eq!(scaled, 1_234_567_890, "Scaled value is incorrect");
}
//
#[test]
fn test_get_domain_separator() {
    let separator = get_domain_separator(Environment::Testnet);
    assert_eq!(
        separator.len(),
        32,
        "Domain separator should be 32 bytes long"
    );
    let expected_separator_hex = "baf501bc2614cf7092d082742580b04c176be1815f46e407eab1bc37ba543c05";
    assert_eq!(
        hex::encode(separator.as_ref()),
        expected_separator_hex,
        "Domain separator does not match expected value"
    );
}

#[test]
fn test_eip712_signature_verification() -> Result<(), Box<dyn std::error::Error>> {
    // Create a wallet
    let wallet: LocalWallet =
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;

    let expected_address = wallet.address();
    println!("Expected signer: {expected_address:?}");

    // Create typed data
    let order = get_test_order();
    // Sign the typed data (synchronous)
    let signature = sign_eip712(&wallet, &order)?;
    println!("Signature: 0x{}", hex::encode(signature.to_vec()));

    // Recover the signer from the signature
    let digest = order.encode_eip712()?;
    let recovered_address = signature.recover(digest)?;

    println!("Recovered signer: {recovered_address:?}");

    // Verify the signer matches
    assert_eq!(
        expected_address, recovered_address,
        "Recovered address doesn't match expected signer"
    );

    println!("✓ Signature verification successful!");

    Ok(())
}

#[test]
fn test_invalid_signature_fails() -> Result<(), Box<dyn std::error::Error>> {
    // Create two different wallets
    let wallet1: LocalWallet =
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;

    let wallet2: LocalWallet =
        "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d".parse()?;

    let order = get_test_order();

    let signature = sign_eip712(&wallet1, &order)?;
    let digest = order.encode_eip712()?;
    let recovered_address = signature.recover(digest)?;

    // Verify it doesn't match wallet2
    assert_ne!(
        wallet2.address(),
        recovered_address,
        "Should not match different wallet"
    );

    println!("✓ Invalid signature correctly rejected!");

    Ok(())
}

#[test]
fn test_known_flow() -> Result<(), Box<dyn std::error::Error>> {
    // Known wallet
    println!("Testing known trade order signing flow...");
    let wallet: LocalWallet =
        "0bb5d63b84421e1268dda020818ae30cf26e7f10e321fb820a8aa69216dea92a".parse()?;
    let expected_address = wallet.address();
    println!("Expected signer: {expected_address:?}");
    // Known order
    let order = get_test_order();

    println!("Order : {order:?}");
    println!("\n");

    let known_signature = "0x82aed7486e9855459f58537e413760597e689d3ba7b859f56b6edc730e044fff2888ccf92cd282a8299d8d6a76f8bf0aa93d97f30340c4bb0d27b626aca62f211b";
    let known_domain_separator = "baf501bc2614cf7092d082742580b04c176be1815f46e407eab1bc37ba543c05";
    // Verify domain separator
    let domain_separator = get_domain_separator(Environment::Testnet);
    println!(
        "Domain Separator: 0x{}",
        hex::encode(domain_separator.as_ref())
    );
    assert_eq!(
        hex::encode(domain_separator.as_ref()),
        known_domain_separator,
        "Domain separator does not match known value"
    );

    let type_hash = TradeOrder::type_hash()?;
    println!("Type Hash: 0x{}", hex::encode(type_hash));

    let msg_hash = order.struct_hash()?;
    println!("Message Hash: 0x{}", hex::encode(msg_hash));

    // get the ful hash

    let full_hash = make_full_hash(&domain_separator, &msg_hash);
    println!("Full Hash: 0x{}", hex::encode(full_hash));

    let known_full_hash = "8400482e02069cefd72a3a86cf96221d75701739b48b3199ce34e7c0525246b7";
    assert_eq!(
        hex::encode(full_hash),
        known_full_hash,
        "Full hash does not match known value"
    );

    // Sign the order
    let signature = sign_eip712(&wallet, &order)?;
    println!("Generated Signature: 0x{}", hex::encode(signature.to_vec()));
    // Verify it matches the known signature
    assert_eq!(
        format!("0x{}", hex::encode(signature.to_vec())),
        known_signature,
        "Generated signature does not match known signature"
    );

    // Recover the signer
    let digest = order.encode_eip712()?;
    let recovered_address = signature.recover(digest)?;
    assert_eq!(
        expected_address, recovered_address,
        "Recovered address doesn't match expected signer"
    );
    println!("✓ Known flow signature verification successful!");
    Ok(())
}
