
use std::env;
use libstripe::Client;
use libstripe::resources::core::product::{ProductType, Products, ProductsParam};
use libstripe::resources::common::currency::Currency;
use libstripe::resources::orders::sku::{InventoryType, Sku, SkuParam, Inventory};

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let client = Client::new(&secret_key);

    let mut products_param = ProductsParam::default();
    products_param.name = Some("Monthly membership base fee");
    products_param.product_type = Some(ProductType::Good);
    products_param.attributes = Some(vec!["gender", "size"]);

    let product = Products::create(&client, products_param)?;

    let mut sku_param = SkuParam::default();
    sku_param.currency = Some(Currency::USD);
    sku_param.attributes = Some({
        let mut attr = std::collections::HashMap::new();
        attr.insert("size", "Medium");
        attr.insert("gender", "Unisex");
        attr
    });

    sku_param.inventory = Some({
        let mut inv = Inventory::default();
        inv.inventory_type = Some(InventoryType::Finite);
        inv.quantity = Some(500);
        inv
    });

    sku_param.price = Some(1500);

    sku_param.product = Some(&product.id);

    let sku = Sku::create(&client, sku_param)?;

    println!("{:?}", sku);

    Ok(())

}