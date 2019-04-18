use libstripe::resources::core::tokens::{TokenParam, Tokens};
use libstripe::resources::paymentmethods::cards::CardParam;
use libstripe::Client;
use std::env;

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let client = Client::new(&secret_key);

    let mut token_param = TokenParam::default();
    token_param.card = Some({
        let mut card_param = CardParam::default();
        card_param.name = Some("John Doe");
        card_param.number = Some("4242424242424242");
        card_param.exp_month = Some("01");
        card_param.exp_year = Some("2021");
        card_param.cvc = Some("123");
        card_param
    });

    let token = Tokens::create(&client, token_param)?;

    println!("{:?}", token);

    Ok(())
}
