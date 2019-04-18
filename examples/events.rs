use libstripe::resources::core::events::Event;
use libstripe::Client;
use std::env;

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let event_id = env::var("EVENT_KEY").expect("Missing 'EVENT_KEY'.");

    let client = Client::new(&secret_key);

    let event = Event::retrieve(&client, &event_id)?;

    println!("{:?}", event);

    Ok(())
}
