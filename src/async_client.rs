use crate::{
    enums::Environment,
    models::{PageOfProductDtos, PageOfSubaccountDtos, ProductDto, SubaccountDto},
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

pub fn get_subaccounts(
    env: Environment,
    sender: &str,
) -> Result<Vec<SubaccountDto>, Box<dyn std::error::Error>> {
    let url: &str = get_server_url(&env);

    let client = reqwest::blocking::Client::new();

    let response = client
        .get(format!("{url}/v1/subaccount"))
        .query(&[("sender", sender)])
        .send()?;

    let subaccount_response: PageOfSubaccountDtos = response.json()?;
    Ok(subaccount_response.data)
}
