use crate::models::{PageOfProductDtos, ProductDto};

const API_URL: &str = "https://api.etherealtest.net";

pub fn get_products() -> Result<Vec<ProductDto>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(format!("{API_URL}/v1/product")).send()?;
    println!("Fetching products");
    let product_response: PageOfProductDtos = response.json()?;
    Ok(product_response.data)
}
