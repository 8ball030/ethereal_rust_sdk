// Golden values captured from the ethers-based signer before the alloy migration. Any change to
// how a message is hashed or signed shows up here as a hex mismatch.
use ethereal_rust_sdk::enums::Environment;
use ethereal_rust_sdk::signable_messages::{
    CancelOrder, ConvertToken, EIP712Auth, ExtendLinkedSigner, InitiateWithdraw, LinkSigner,
    RefreshLinkedSigner, RevokeLinkedSigner, TradeOrder,
};
use ethereal_rust_sdk::signing::{get_domain_separator, hex_to_bytes32, Eip712};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Address;
use ethers::utils::hex;

const KEY: &str = "0bb5d63b84421e1268dda020818ae30cf26e7f10e321fb820a8aa69216dea92a";

fn wallet() -> LocalWallet {
    KEY.parse().unwrap()
}

fn sign_hex<T: Eip712>(msg: &T, env: Environment) -> String {
    hex::encode(msg.sign(env, &wallet()).unwrap().to_vec())
}

fn addr(s: &str) -> Address {
    s.parse().unwrap()
}

fn bytes32(s: &str) -> [u8; 32] {
    hex_to_bytes32(s).unwrap()
}

struct Golden {
    type_hash: &'static str,
    struct_hash: &'static str,
    testnet_digest: &'static str,
    testnet_sig: &'static str,
    mainnet_sig: &'static str,
}

fn check<T: Eip712>(name: &str, msg: &T, g: Golden) {
    let actual = [
        hex::encode(T::type_hash().unwrap()),
        hex::encode(msg.struct_hash().unwrap()),
        hex::encode(msg.encode_eip712(Environment::Testnet).unwrap()),
        sign_hex(msg, Environment::Testnet),
        sign_hex(msg, Environment::Mainnet),
    ];
    println!("{name}: {actual:#?}");
    let expected = [
        g.type_hash,
        g.struct_hash,
        g.testnet_digest,
        g.testnet_sig,
        g.mainnet_sig,
    ];
    assert_eq!(actual, expected, "{name}");
}

const SENDER: &str = "0xdeadbeef00000000000000000000000000000001";
const OTHER: &str = "0x00000000000000000000000000000000cafebabe";
const SUBACCOUNT: &str = "0x123456789abcde00000000000000000000000000000000000000000000000000";

#[test]
fn domain_separators() {
    assert_eq!(
        hex::encode(get_domain_separator(Environment::Testnet)),
        "f45e2c25657b6ad7b4835073c1c65bec7f6b973d6064b491dd248315296e7d6a"
    );
    assert_eq!(
        hex::encode(get_domain_separator(Environment::Mainnet)),
        "8692fb170e27b3ec004ef5be36e41ce562ad15f81c0de89a6f78b7fcde7ca4f5"
    );
}

// The API compares these strings, so their exact rendering is part of the wire contract.
#[test]
fn address_rendering() {
    assert_eq!(
        format!("{:?}", wallet().address()),
        "0x9e4f7e220113d0b12be4996ad8b73a09aaacdb2c"
    );
    assert_eq!(
        format!("{:?}", addr(SENDER)),
        "0xdeadbeef00000000000000000000000000000001"
    );
}

#[test]
fn trade_order() {
    let msg = TradeOrder {
        sender: addr(SENDER),
        subaccount: bytes32(SUBACCOUNT),
        quantity: 1_000_000_000,
        price: u128::MAX,
        reduce_only: true,
        side: 1,
        engine_type: 255,
        product_id: u32::MAX,
        nonce: 1764897077655477722,
        signed_at: 1764897077,
    };
    check(
        "TradeOrder",
        &msg,
        Golden {
            type_hash: "b5d1ecfc4cd1b5349a6c9aad859972358d2703e5c438a4dd6ade4b2b2e502651",
            struct_hash: "643b1695f6ced91d33143f4148b73c5d4573bf6b71a34bb5d39ec9e6d30f323e",
            testnet_digest: "737f08123c0ed536265d89419ab5eba41f4b6950797e0db49ccff158d6ce2afa",
            testnet_sig: "bdde45227b586231c317cabb26375899afc20e513524bb8832ef0958c65203122a7886bfc987cfb6d86495a6ca23d8b6ef322f0162b9b912a7066460806f10f01c",
            mainnet_sig: "a7964010dc55c0d2c4d9cc13a7a8b5698bdb98b5065edd93b0faf7d9272b8d806611be8b52d61ecbca52964f8acffdbcf4ce181fda372ffb9739936071c541d21c",
        },
    );
}

#[test]
fn cancel_order() {
    let msg = CancelOrder {
        sender: addr(SENDER),
        subaccount: bytes32(SUBACCOUNT),
        nonce: u64::MAX,
    };
    check(
        "CancelOrder",
        &msg,
        Golden {
            type_hash: "261ced7eb7c78527bff48f58be621640dd4739c8bb3d566b52f295a6b50b8ee5",
            struct_hash: "32b92c500dc6ee5cf3da2b7aedf71c6b4c64ba9cd77dc34453de554d1c88adca",
            testnet_digest: "788c7befdf991f9d116f7281f006c70cbe4c832c818417b555bbe633b6ae7cd1",
            testnet_sig: "b2466c0c283b24f724d3732a2f15fdf10e9801b183c9f25d132d54fd63a6305319a278a78c7368237b54b760e16aa5adddd4f709d0ea1af825891c2322bfe4891b",
            mainnet_sig: "eb86990e906421fa3d6b1eba32ce7493b0c47bf1debe75e3849ea982fda22801582f8f2955edc41c49dedea5955d5b9ae7b388ae7d32b4413266d1cf785e4b9a1b",
        },
    );
}

#[test]
fn link_signer() {
    let msg = LinkSigner {
        sender: addr(SENDER),
        signer: addr(OTHER),
        subaccount: bytes32(SUBACCOUNT),
        nonce: 42,
        signed_at: 1764897077,
    };
    check(
        "LinkSigner",
        &msg,
        Golden {
            type_hash: "569a9ce371b68570aa9fb81967328abd23933f334898556086c45a9d69a65c63",
            struct_hash: "d660efade6b1675ab1f7c499711dba9f52f9735b23b3b1cfab29ac2be51e8425",
            testnet_digest: "9a2fc62ddc6f09550ec07242bd01e15e62f830c05688df8ed29bc9f4a8f1ea12",
            testnet_sig: "0e1bd3f8e8132a4279c5fe1947b5856b06fefecbbb8374312709e2175ec890b93590fcbc89413d91e9f0d93b449f8ae36b1a72839eab0633cdfaf8057924c7931c",
            mainnet_sig: "5a1f726ffb6703277130f3d4118e37d2f934f543fb3a85527d62dea330aaa3b970358d88e9c9418f0a13844e1517c8e15f84597cc97f875b8ac6d1bcf3fb6c881b",
        },
    );
}

#[test]
fn revoke_linked_signer() {
    let msg = RevokeLinkedSigner {
        sender: addr(SENDER),
        signer: addr(OTHER),
        subaccount: bytes32(SUBACCOUNT),
        nonce: 43,
        signed_at: 1764897078,
    };
    check(
        "RevokeLinkedSigner",
        &msg,
        Golden {
            type_hash: "95676cebee29a8d643b04e0fec7c0af0b3c59e6d80b7b72b8fb76f2bfc720d42",
            struct_hash: "3d23f9a55421305b2e5129873bdab855589151327cce9a2671f4199a986496b6",
            testnet_digest: "2dbae6cbbe6c295bfd323354fdcf2a9b2fdb693092fb8f597d2dcdab00feb317",
            testnet_sig: "51f9d2fe5be743595ea80c26a5b9e25dbb42d98d3d052802cd8e0522151f2d4f1d5b4088faf13fe3fc8e401a20cad79eb5651c15bd8ab5837fe0fa9ffca72fd51c",
            mainnet_sig: "8e40948ba0fb8f765cc13c5b337e32293a0050270f2956871c257cc472c7ccd355df8b9f28c3a3160259357cc051fc4aa54a13a3fa833cbedc63972aa6103ddc1c",
        },
    );
}

#[test]
fn refresh_linked_signer() {
    let msg = RefreshLinkedSigner {
        sender: addr(SENDER),
        signer: addr(OTHER),
        nonce: 44,
        signed_at: 1764897079,
    };
    check(
        "RefreshLinkedSigner",
        &msg,
        Golden {
            type_hash: "26ce72cd63f2e5f3234e5693b1d7f31de5fcbd45212268c68d95c8e076f27da4",
            struct_hash: "45b98159067e38a805b254aa8565e5d71faef91f5bce712e8df122cbed9b21f5",
            testnet_digest: "83aaebb249036e87cea6d09d3e02d6d360e37729278fd5c4d2d76e2965d0f402",
            testnet_sig: "24922135440322316d7f037af584f80e08934102bc05468e994c2d04b94623ed704e7f1a00c566c87b4e576201a33d93208dfb3453f08cd741c4e15a14541ac01b",
            mainnet_sig: "6203eea732cfb2dd5f240c29c9be9edd071b2be321dbb2d981e8b76a2dceebea1f9a53deefb8dc74af659aa38cbc98987102c7303f8865ba703228d393bd2e2d1c",
        },
    );
}

#[test]
fn extend_linked_signer() {
    let msg = ExtendLinkedSigner {
        sender: addr(SENDER),
        nonce: 45,
        signed_at: 1764897080,
    };
    check(
        "ExtendLinkedSigner",
        &msg,
        Golden {
            type_hash: "5e23950eb3a5acbc76e82bd42c5de31950e80137fee3adc0899d006ff2e512d1",
            struct_hash: "d33b4ca949f02d99a818236568b11562a9b5c954b05ed1c1bc5b43307116f75c",
            testnet_digest: "9f15fd60caec5b0428faeb806880721f549f1732cf4b1aa34ea846896f1a14aa",
            testnet_sig: "aed0f1360bf6eec1089262929c6a596aabeaaf39b27e12f87bf9725f089779243b81b388388240e2e4a48cc171ec3a4264b77c508f7c44398d8870e95ee934cf1b",
            mainnet_sig: "316d152fa796895451d279cc2481fe01a711db88dc7fcf58fe4ca3eddc162c5f24d6fd4eedd0e51307435ce31e87cc7f45088fb01c4f5d45dd43c2e2ae8bdc661b",
        },
    );
}

#[test]
fn initiate_withdraw() {
    let msg = InitiateWithdraw {
        account: addr(SENDER),
        subaccount: bytes32(SUBACCOUNT),
        token: addr(OTHER),
        amount: u128::MAX - 1,
        nonce: 46,
        signed_at: 1764897081,
        destination_address: bytes32(
            "0xff000000000000000000000000000000000000000000000000000000000000ee",
        ),
        destination_endpoint_id: 30101,
    };
    check(
        "InitiateWithdraw",
        &msg,
        Golden {
            type_hash: "a2e64d6fddb0ebb14fd1038d1434a9f020bbd8274e414dc9f08d361b650038db",
            struct_hash: "e619b6d9e728b3a3dc287c8d7c4ccdda23a1dda54aa4b5f2413a6e65dd1d8289",
            testnet_digest: "bdd8a2ae320afcf71a1cec3444e65dcacd55f3511235ee2fe5f2e5d38d3eb6ca",
            testnet_sig: "5206ae269972260e54ebf17b4246690538756ad3f33dbe8a860ba61bafae79265eb19ddddc48708425b9a131d8c4fe8def60055f231a49e35919088b32ed695d1b",
            mainnet_sig: "9099f855da1c829d6aec270c5c044934a5ca3457c92606ca1419113381f2ff70114c510fe7a147794711b02fc3e9aab17fd58d310276afce75365be0e269b0501b",
        },
    );
}

#[test]
fn convert_token() {
    let msg = ConvertToken {
        sender: addr(SENDER),
        subaccount: bytes32(SUBACCOUNT),
        from_token: addr(OTHER),
        to_token: addr("0x1111111111111111111111111111111111111111"),
        amount: 123_456_789_000_000_000,
        nonce: 47,
        signed_at: 1764897082,
    };
    check(
        "ConvertToken",
        &msg,
        Golden {
            type_hash: "40797ca791cf614be9b93e9c6266836953953f6fae98342a9a07367730942870",
            struct_hash: "895db2e9d92e9b770fd39b00d2daf296d2e02b3921700a27d4ff5b2679dd3bff",
            testnet_digest: "072c029884ae7a9bcde2fa3973773e6c5a18dc12cca31d981765e81660c0ee90",
            testnet_sig: "11c552a15f53712e84cf630aeea05fe768fa07d8f72b484a95055a41b1a8422804011af4e9f022c2c2c5ccf4f5f3ff1d9c3b8f6430e07d815f7070a3d4c4260b1c",
            mainnet_sig: "a9d95a344669f75c7824844110d11ffd2a17311a572d3b06f5b6716353813d2b25ca4847d9297694fc7e467b2416f0d4b2c7065644582368d731f1929a38aa901b",
        },
    );
}

#[test]
fn eip712_auth() {
    let msg = EIP712Auth {
        sender: addr(SENDER),
        intent: 7,
        signed_at: 1764897083,
    };
    check(
        "EIP712Auth",
        &msg,
        Golden {
            type_hash: "9c0fdb497094bd5c03b6df2c4189af2e019b051706d6de189a13f6db757850e2",
            struct_hash: "3c90610e8059263fcf991bcc84a9a3b43e8f0f77e04425d6ca4a76d4c5d10d88",
            testnet_digest: "9868e7603e4b709305db37f19efa64cc174f8dcfbc020ab5e3ea862aa44b2b34",
            testnet_sig: "5b3cfba9310324d7ac94bc4c951c4eb22f401f1968c5a3e96932285cba7a80ad1922882ec272f9590256bc49ec2ac9437a01798f58796dd8c4c4cce95e3fb0a31c",
            mainnet_sig: "ab9caefe3c67d1ff8976d6433ae66f5328409f8240a5b81d1af797ad5591a1c90f3d21de1982dca5ade96fc3d3bb121381c31ea49d774ee5a49d7556541a80df1c",
        },
    );
}
