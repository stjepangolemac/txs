use rust_decimal::Decimal;
use std::collections::HashMap;

struct MessageData {
    client: u16,
    tx: u32,
    amount: Option<Decimal>,
}

enum Transaction {
    Deposit(MessageData),
    Withdrawal(MessageData),
    Dispute(MessageData),
    Resolve(MessageData),
    Chargeback(MessageData),
}

struct Account {
    available: Decimal,
    held: Decimal,
    total: Decimal,
    locked: bool,
}

pub struct Processor {
    accounts: HashMap<u16, Account>,
    transactions: HashMap<u32, Transaction>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn process(&mut self, transaction: Transaction) {
        todo!()
    }

    pub fn snapshot(&self) -> &HashMap<u16, Account> {
        todo!()
    }
}
