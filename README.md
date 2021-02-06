# Behaviors

I don't have a lot of experience with business rules of disputes and chargebacks so it's possible I've made a mistake. Here are the things I implemented even though I'm not sure that they're right.

- You can only dispute a deposit (disputing a withdrawal would invent money when it's already gone, maybe that is okay?)
- Dispute can push the account balance into negative (I suppose that's the risk a company needs to take, not sure)
- Resolves and chargebacks will fail if there is not enough held funds (this probably means there's a bug in the system)

# Potential improvements

- Different transaction types could have different types for their data, that would provide even better safety guarantees but I've opted for a simpler version here as this is just an example
- I've used anyhow to simplify error handling but a more sophisticated error type can be used to differentiate between different error variants
