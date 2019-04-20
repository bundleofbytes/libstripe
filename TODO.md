Update prelude so users do not have to manually import modules

Add documentation to this library.

~~Remove StripeService trait and revert to having the api url being within the Client~~
- ~~The idea behind the StripeService trait was to prevent any junk data passing through and to allow changing the uri when it comes to making any request outside of `https://api.stripe.com/`, but now as I review the code during the structure change of this library, I no longer think it is necessary to have that trait in place. Removing it would not be a breaking change since it is an internal change so it could easily happen in the future without one needing to change something on their end unless they are implementing the trait directly (which is not the intented use case for the end developer).~~

Implement async into Client
- This was put on the backburner since I havent had the time to complete the implementation, but this is something to look into. The only roadblock I see would be related to multipart. I do know reqwest have different internals for multipart when it comes to async. Might have to restrict file uploading to outside of async for the time being until further investigation is done.

Write a tool to generate code based on schema for both request and response
- This will make it easier to keep up with any future API Updates from stripe especially if its a change that would likely break
  this library backwards compatibility. Something like this could also make it effective to support different api versions from stripe.

Look into the benefits of migrating [back] to hyper
- Initially, when this library first was written, it used hyper at the time (before tokio was implemented). I moved from hyper to reqwest to simplify http request due to the unstable nature of hyper at the time. Considering hyper has been reasonably stable for awhile, it might be better to look into benefits of moving back. However, this wont be for done for awhile so I this is not important.