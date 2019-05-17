libstripe
===========

![Status](https://img.shields.io/badge/status-active-brightgreen.svg?style=flat)
[![](http://meritbadge.herokuapp.com/libstripe)](https://crates.io/crates/libstripe)
[![Build Status](https://api.cirrus-ci.com/github/bundleofbytes/libstripe.svg)](https://cirrus-ci.com/github/bundleofbytes/libstripe)

Stripe library for rust.

Note: Everything is subject to change but everything should be stable to use.

## Example

```rust
use libstripe::Client;
use libstripe::resources::core::customer::{Customer, CustomerParam};

fn main() {
    let client = Client::new("sk_test_..............");

    let mut param = CustomerParam::default();

    param.email = Some("example@example.com");
    param.description = Some("Example account");

    let customer = match Customer::create(&client, param) {
        Ok(cust) => cust,
        Err(e) => panic!("{}", e)
    };

    println!("{:?}", customer);

}
```
