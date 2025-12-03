use crate::{
    enums::Environment,
    models::{PageOfProductDtos, ProductDto},
};

fn get_server_url(environment: &Environment) -> &str {
    match environment {
        Environment::Production => "https://api.ethereal.trade",
        Environment::Testnet => "https://api.etherealtest.net",
    }
}
pub fn get_products(env: Environment) -> Result<Vec<ProductDto>, Box<dyn std::error::Error>> {
    let url: &str = get_server_url(&env);

    let client = reqwest::blocking::Client::new();
    let response = client.get(format!("{url}/v1/product")).send()?;
    println!("Fetching products");
    let product_response: PageOfProductDtos = response.json()?;
    Ok(product_response.data)
}
