use ethereal_rust_sdk::{async_client::client::HttpClient, enums::Environment, models::ProductDto};

pub async fn create_test_client() -> anyhow::Result<HttpClient> {
    let env = Environment::Testnet;
    let private_key = "0bb5d63b84421e1268dda020818ae30cf26e7f10e321fb820a8aa69216dea92a";
    let http_client = HttpClient::new(env, private_key).await;
    Ok(http_client)
}

pub async fn get_product(client: &HttpClient) -> anyhow::Result<ProductDto> {
    let params = ethereal_rust_sdk::apis::product_api::ProductControllerListParams::default();
    let products = client.product().list(params).await?;
    let product = products.data.first().unwrap().clone();
    Ok(product)
}

#[tokio::test]
async fn test_create_test_client() {
    let client = create_test_client().await;
    assert!(client.is_ok());
}

#[tokio::test]
async fn test_get_product() {
    let client = create_test_client().await.unwrap();
    let product = get_product(&client).await;
    assert!(product.is_ok());
}
