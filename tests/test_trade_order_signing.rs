// Simple tests to ensure that the steps of the trade order signing process work as expected.
use ethereal_rust_sdk::signing::{TradeOrder, encode_eip712_message, get_domain_separator, to_scaled_e9};
use ethers::types::{Address, H256, U128};
use ethers::signers::{LocalWallet, Signer};
use ethereal_rust_sdk::enums::Environment;
use ethers::utils::hex;


fn get_test_wallet() -> LocalWallet {
    "0bb5d63b84421e1268dda020818ae30cf26e7f10e321fb820a8aa69216dea92a"
        .parse::<LocalWallet>()
        .expect("Failed to create test wallet")
}

fn get_test_subaccount() -> H256 {
    "0x0000000000000000000000000000000000000000000000000000000000000000"
        .parse()
        .expect("Failed to parse test subaccount")
}

fn get_nonce() -> u64 {
    1 // Fixed nonce for testing
}
fn get_now() -> u64 {
    1_600_000_000 // Fixed timestamp for testing
}

fn get_test_trade_order(sender: Address, subaccount: H256) -> TradeOrder {
    TradeOrder {
        sender,
        subaccount,
        quantity: U128::from(to_scaled_e9(0.001)),
        price: U128::from(to_scaled_e9(80000.0)),
        reduce_only: false,
        side: 0,
        engine_type: 0,
        product_id: 1,
        nonce: get_nonce(),
        signed_at: get_now(),
    }
}

#[test]
fn test_to_scaled_e9() {
    let value = 1.23456789;
    let scaled = to_scaled_e9(value);
    assert_eq!(scaled, 1_234_567_890, "Scaled value is incorrect");
}

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
fn test_trade_order_signing() {
    let wallet = get_test_wallet();
    let sender_address = wallet.address();
    let subaccount = get_test_subaccount();
    let trade_order = get_test_trade_order(sender_address, subaccount);
    let encoded_message = encode_eip712_message(&trade_order);
    let signature = wallet
        .sign_hash(encoded_message)
        .expect("Failed to sign trade order");
    assert!(!signature.to_string().is_empty(), "Signature should not be empty");
    let expected_signature = "4a2c2c69e7b075814ead775c9a5ee9e6e6d4f8270ce28d29fc5c7b335f553db01297e16dd985296f5dd21c01468a0548d9eb2e660b2a06c120a39a1b14cc96451c";
    assert_eq!(
        signature.to_string(),
        expected_signature,
        "Signature does not match expected value"
    );
}


