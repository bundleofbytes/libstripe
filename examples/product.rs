use libstripe::resources::core::product::{ProductType, Products, ProductsParam};
use libstripe::Client;
use std::env;

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let client = Client::new(&secret_key);

    let mut products_param = ProductsParam::default();
    products_param.name = Some("Monthly membership base fee");
    products_param.product_type = Some(ProductType::Service);

    let product = Products::create(&client, products_param)?;

    println!("{:?}", product);

    Ok(())
}
