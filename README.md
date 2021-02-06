# Instructions

There are a few sample csv files in the `data/` dir and they can be processed by doing:

```
cargo run -- ./data/sample.csv
```

Common transaction combinations and behaviors are fully tested which can be checked by doing:

```
cargo test
```

# Behaviors

I don't have a lot of experience with business rules of disputes and chargebacks so it's possible I've made a mistake. Here are the things I implemented even though I'm not sure that they're right.

- Only deposits can create new accounts if the client is unknown
- Withdrawals, disputes, resolves, and chargebacks to a nonexistent account don't change the system state, but won't error (maybe they should for logging purposes?)
- You can only dispute a deposit (disputing a withdrawal would invent money when it's already gone, maybe that is okay?)
- Dispute can push the available account balance into negative (I suppose that's the risk a company needs to take, not sure)
- Resolves and chargebacks will fail if there is not enough held funds (this probably means there's a bug in the system)

# Potential improvements

- Different transaction types could have different types for their data, that would provide even better safety guarantees but I've opted for a simpler version here as this is just an example
- I've used anyhow to simplify error handling but a more sophisticated error type can be used to differentiate between different error variants
- Withdrawals, disputes, resolves, and chargebacks could return a Result<Option<()>> to inform the caller if the system state was changed (I decided not to do that here but it would be a small change)
- Invalid transactions will produce an errors but they won't affect the application, this is very easy to change (partner errors are the only ones mentioned in the spec and should be ignored)
