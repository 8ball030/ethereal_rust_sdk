use ethereal_rust_sdk::enums::Environment;
use ethereal_rust_sdk::signable_messages::TradeOrder;
use ethereal_rust_sdk::signing::Eip712;
use ethereal_rust_sdk::signing::{
    get_domain_separator, hex_to_bytes32, make_full_hash, to_scaled_e9,
};
use ethers::signers::{LocalWallet, Signer};
use ethers::utils::hex;
use rust_decimal_macros::dec;

fn get_test_order() -> TradeOrder {
    TradeOrder {
        sender: "0xdeadbeef00000000000000000000000000000000"
            .parse()
            .unwrap(),
        subaccount: hex_to_bytes32(
            "0x123456789abcde00000000000000000000000000000000000000000000000000",
        )
        .unwrap(),
        quantity: to_scaled_e9(dec!(1.0)).expect("Failed to scale quantity"),
        price: to_scaled_e9(dec!(3000.0)).expect("Failed to scale price"),
        reduce_only: false,
        side: 0,
        engine_type: 0,
        product_id: 2,
        nonce: 1764897077655477722,
        signed_at: 1764897077,
    }
}

#[test]
fn test_to_scaled_e9() {
    let value = dec!(1.23456789);
    let scaled = to_scaled_e9(value).unwrap();
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
    let expected_separator_hex = "f45e2c25657b6ad7b4835073c1c65bec7f6b973d6064b491dd248315296e7d6a";
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
    let env = Environment::Testnet;
    println!("Expected signer: {expected_address:?}");

    // Create typed data
    let order = get_test_order();
    // Sign the typed data (synchronous)
    let signature = order.sign(env, &wallet)?;
    println!("Signature: 0x{}", hex::encode(signature.to_vec()));

    // Recover the signer from the signature
    let digest = order.encode_eip712(env)?;
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

    let env = Environment::Testnet;
    let order = get_test_order();

    let signature = order.sign(env, &wallet1)?;
    let digest = order.encode_eip712(env)?;
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
    let env = Environment::Testnet;
    let wallet: LocalWallet =
        "0bb5d63b84421e1268dda020818ae30cf26e7f10e321fb820a8aa69216dea92a".parse()?;
    let expected_address = wallet.address();
    println!("Expected signer: {expected_address:?}");
    // Known order
    let order = get_test_order();

    println!("Order : {order:?}");
    println!("\n");

    let known_signature = "0xbc76ecd2bcd2bab0c6dc989d238bcd0216c4b574b2de0d0141ae205aabdd53532206441c3593e34de96cbe2e1bff501095a73cb2f2dcb05d1e2b03f2db6b24461c";
    let known_domain_separator = "f45e2c25657b6ad7b4835073c1c65bec7f6b973d6064b491dd248315296e7d6a";
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

    let known_full_hash = "f5183bf5a48ce9ace17747133ac563a4273981d32a72ef74d1b66a6447c34ddc";
    assert_eq!(
        hex::encode(full_hash),
        known_full_hash,
        "Full hash does not match known value"
    );

    // Sign the order
    let signature = order.sign(env, &wallet)?;
    println!("Generated Signature: 0x{}", hex::encode(signature.to_vec()));
    // Verify it matches the known signature
    assert_eq!(
        format!("0x{}", hex::encode(signature.to_vec())),
        known_signature,
        "Generated signature does not match known signature"
    );

    // Recover the signer
    let digest = order.encode_eip712(env)?;
    let recovered_address = signature.recover(digest)?;
    assert_eq!(
        expected_address, recovered_address,
        "Recovered address doesn't match expected signer"
    );
    println!("✓ Known flow signature verification successful!");
    Ok(())
}

// ── to_scaled_e9 and hex_to_bytes32 report their failures instead of panicking ───────────────

#[test]
fn scaling_a_negative_value_is_an_error_not_a_panic() {
    let err = to_scaled_e9(dec!(-0.5)).unwrap_err().to_string();
    assert!(err.contains("unsigned"), "{}", err);
    assert_eq!(to_scaled_e9(dec!(0)).unwrap(), 0, "zero is fine");
    assert_eq!(to_scaled_e9(dec!(-0.0)).unwrap(), 0, "negative zero is zero");
}

#[test]
fn scaling_a_value_too_large_for_the_wire_is_an_error_not_a_panic() {
    let err = to_scaled_e9(rust_decimal::Decimal::MAX).unwrap_err().to_string();
    assert!(err.contains("does not fit"), "{}", err);
}

#[test]
fn a_32_byte_hex_string_decodes_with_or_without_the_prefix() {
    let hex32 = "123456789abcde00000000000000000000000000000000000000000000000000";
    let with = hex_to_bytes32(&format!("0x{hex32}")).unwrap();
    let without = hex_to_bytes32(hex32).unwrap();
    assert_eq!(with, without);
    assert_eq!(with[0], 0x12);
}

#[test]
fn a_short_or_long_hex_string_is_an_error_instead_of_being_padded_or_cut() {
    let short = hex_to_bytes32("0x1234").unwrap_err().to_string();
    assert!(short.contains("expected 32 bytes") && short.contains("2 byte(s)"), "{}", short);
    let long = hex_to_bytes32(&format!("0x{}", "ab".repeat(33))).unwrap_err().to_string();
    assert!(long.contains("33 byte(s)"), "{}", long);
    assert!(hex_to_bytes32("0xzz").is_err(), "not hex at all");
}
