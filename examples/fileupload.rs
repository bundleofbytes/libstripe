use libstripe::resources::core::file::{File, FileLinkDataParam, FilePurpose};
use libstripe::Client;
use std::env;

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let filename = env::var("FILENAME").expect("Missing 'FILENAME'.");
    let client = Client::new(&secret_key);

    let fileupload = File::create(
        &client,
        &filename,
        FilePurpose::DisputeEvidence,
        FileLinkDataParam::default(),
    )?;

    println!("{:?}", fileupload);

    Ok(())
}
