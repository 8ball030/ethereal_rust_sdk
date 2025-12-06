use ethereal_rust_sdk::{enums::Environment, sync_client::client::HttpClient};

pub fn create_test_client() -> anyhow::Result<HttpClient> {
    let env = Environment::Testnet;
    let private_key = "0bb5d63b84421e1268dda020818ae30cf26e7f10e321fb820a8aa69216dea92a";
    let http_client = HttpClient::new(env, private_key);
    Ok(http_client)
}
