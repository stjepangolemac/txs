use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type ClientId = u16;
type TransactionId = u32;

#[derive(Debug, Deserialize)]
pub enum MessageType {
    #[serde(alias = "deposit")]
    Deposit,

    #[serde(alias = "withdrawal")]
    Withdrawal,

    #[serde(alias = "dispute")]
    Dispute,

    #[serde(alias = "resolve")]
    Resolve,

    #[serde(alias = "chargeback")]
    Chargeback,
}

#[derive(Debug, Deserialize)]
pub struct Message(MessageType, ClientId, TransactionId, Option<Decimal>);

#[derive(Debug, Deserialize)]
struct TransactionData {
    client: ClientId,
    tx: TransactionId,
    amount: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
enum Transaction {
    Deposit(TransactionData),
    Withdrawal(TransactionData),
    Dispute(TransactionData),
    Resolve(TransactionData),
    Chargeback(TransactionData),
}

#[derive(Debug, Deserialize, Serialize)]
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

    pub fn process(&mut self, message: Message) {
        todo!()
    }

    pub fn snapshot(&self) -> &HashMap<u16, Account> {
        todo!()
    }
}
