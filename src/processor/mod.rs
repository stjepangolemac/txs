use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod behaviors;

pub type ClientId = u16;
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
pub struct TransactionData {
    client: ClientId,
    transaction: TransactionId,
    amount: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub enum Transaction {
    Deposit(TransactionData),
    Withdrawal(TransactionData),
    Dispute(TransactionData),
    Resolve(TransactionData),
    Chargeback(TransactionData),
}

impl From<Message> for Transaction {
    fn from(message: Message) -> Self {
        let Message(message_type, client, transaction, amount) = message;

        let data = TransactionData {
            client,
            transaction,
            amount: amount.map(|mut num| {
                num.rescale(4);

                num
            }),
        };

        match message_type {
            MessageType::Deposit => Transaction::Deposit(data),
            MessageType::Withdrawal => Transaction::Withdrawal(data),
            MessageType::Dispute => Transaction::Dispute(data),
            MessageType::Resolve => Transaction::Resolve(data),
            MessageType::Chargeback => Transaction::Chargeback(data),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub available: Decimal,
    pub held: Decimal,
    pub frozen: bool,
}

impl Account {
    pub fn total(&self) -> Decimal {
        self.available + self.held
    }
}

pub type Accounts = HashMap<u16, Account>;

pub type Disputed = bool;
pub type Transactions = HashMap<u32, (Transaction, Disputed)>;

pub struct Processor {
    accounts: Accounts,
    transactions: Transactions,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn process(&mut self, message: Message) {
        let transaction_id = message.2;
        let transaction: Transaction = message.into();

        let res = match transaction {
            Transaction::Deposit(ref data) => behaviors::deposit(&data, &mut self.accounts),
            Transaction::Withdrawal(ref data) => behaviors::withdrawal(data, &mut self.accounts),
            Transaction::Dispute(ref data) => {
                behaviors::dispute(data, &mut self.accounts, &mut self.transactions)
            }
            Transaction::Resolve(ref data) => {
                behaviors::resolve(data, &mut self.accounts, &mut self.transactions)
            }
            Transaction::Chargeback(ref data) => {
                behaviors::chargeback(data, &mut self.accounts, &mut self.transactions)
            }
        };

        let was_deposit = matches!(transaction, Transaction::Deposit(_));
        let was_ok = res.is_ok();

        if was_deposit && was_ok {
            self.transactions
                .insert(transaction_id, (transaction, false));
        }
    }

    pub fn snapshot(&self) -> &HashMap<u16, Account> {
        &self.accounts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::*;

    fn get_processed_snapshot(messages: Vec<Message>) -> Accounts {
        let mut processor = Processor::new();

        messages
            .into_iter()
            .for_each(|message| processor.process(message));

        processor.accounts
    }

    #[test]
    fn deposits() {
        let messages = {
            use MessageType::*;

            vec![
                Message(Deposit, 1, 1, Some(dec!(10))),
                Message(Deposit, 2, 2, Some(dec!(5))),
                Message(Deposit, 1, 3, Some(dec!(3))),
            ]
        };

        let snapshot = get_processed_snapshot(messages);

        let account1 = snapshot.get(&1).unwrap();
        assert_eq!(account1.available, dec!(13));

        let account2 = snapshot.get(&2).unwrap();
        assert_eq!(account2.available, dec!(5));
    }

    #[test]
    fn withdrawals() {
        let messages = {
            use MessageType::*;

            vec![
                Message(Deposit, 1, 1, Some(dec!(10))),
                Message(Withdrawal, 1, 2, Some(dec!(3))),
            ]
        };

        let snapshot = get_processed_snapshot(messages);

        let account1 = snapshot.get(&1).unwrap();
        assert_eq!(account1.available, dec!(7));
    }

    #[test]
    fn dispute() {
        let messages = {
            use MessageType::*;

            vec![
                Message(Deposit, 1, 1, Some(dec!(10))),
                Message(Deposit, 1, 2, Some(dec!(5))),
                Message(Dispute, 1, 2, None),
            ]
        };

        let snapshot = get_processed_snapshot(messages);

        let account1 = snapshot.get(&1).unwrap();
        assert_eq!(account1.available, dec!(10));
        assert_eq!(account1.held, dec!(5));
    }

    #[test]
    fn resolves() {
        let messages = {
            use MessageType::*;

            vec![
                Message(Deposit, 1, 1, Some(dec!(10))),
                Message(Deposit, 1, 2, Some(dec!(5))),
                Message(Dispute, 1, 2, None),
                Message(Resolve, 1, 2, None),
            ]
        };

        let snapshot = get_processed_snapshot(messages);

        let account1 = snapshot.get(&1).unwrap();
        assert_eq!(account1.available, dec!(15));
        assert_eq!(account1.held, dec!(0));
    }

    #[test]
    fn chargebacks() {
        let messages = {
            use MessageType::*;

            vec![
                Message(Deposit, 1, 1, Some(dec!(10))),
                Message(Deposit, 1, 2, Some(dec!(5))),
                Message(Dispute, 1, 2, None),
                Message(Chargeback, 1, 2, None),
            ]
        };

        let snapshot = get_processed_snapshot(messages);

        let account1 = snapshot.get(&1).unwrap();
        assert_eq!(account1.available, dec!(10));
        assert_eq!(account1.held, dec!(0));
        assert!(account1.frozen);
    }
}
