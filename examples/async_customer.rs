
use libstripe::resources::core::customer::{Customer, CustomerParam};
use libstripe::Client;
use futures::future::{self, Future};
use std::env;

fn main() {
    tokio::run(future::lazy(|| {
        let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");

        let client = Client::new(&secret_key);

        let mut param = CustomerParam::default();
        param.email = Some("j.doe@example.com");
        param.description = Some("Example Account");

        Customer::create(&client, param)
            .and_then(move |customer| {
                println!("{:?}", customer);
                Customer::delete(&client, &customer.id)
                    .map(|deleted| {
                        println!("{:?}",  deleted);
                    })
            })
            .map_err(|e| println!("Error processing request: {:?}", e))
    }));
}
