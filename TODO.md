Update prelude so users do not have to manually import modules

Remove StripeService trait and revert to having the api url being within the Client
- The idea behind the StripeService trait was to prevent any junk data passing through and to allow changing the uri when it comes to making any request outside of `https://api.stripe.com/`, but now as I review the code during the structure change of this library, I no longer think it is necessary to have that trait in place. Removing it would not be a breaking change since it is an internal change so it could easily happen in the future without one needing to change something on their end unless they are implementing the trait directly (which is not the intented use case for the end developer).

Implement async into Client

Write a tool to generate code based on schema for both request and response
- This will make it easier to keep up with any future API Updates from stripe especially if its a change that would likely break
  this library backwards compatibility. Something like this could also make it effective to support different api versions from stripe.

