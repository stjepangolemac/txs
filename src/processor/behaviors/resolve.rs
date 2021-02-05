use crate::processor::{Account, ClientId, Transaction, TransactionData, TransactionId};
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub fn resolve(
    data: &TransactionData,
    accounts: &mut HashMap<ClientId, Account>,
    transactions: &HashMap<TransactionId, Transaction>,
) -> Result<()> {
    let TransactionData {
        client,
        transaction,
        ..
    } = data;

    if let Some(referenced_transaction) = transactions.get(transaction) {
        let amount = match referenced_transaction {
            Transaction::Deposit(data) => data
                .amount
                .ok_or_else(|| anyhow!("Referenced deposit should have the amount")),
            _ => Err(anyhow!("Cannot resolve non deposit")),
        }?;

        accounts.entry(*client).and_modify(|account| {
            let not_frozen = !account.frozen;
            let has_held_funds = account.held >= amount;

            if not_frozen && has_held_funds {
                account.available += amount;
                account.held -= amount;
            }
        });

        return Ok(());
    }

    Err(anyhow!("Cannot dispute unknown transaction"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::*;

    #[test]
    fn resolution_works() {
        let client = 1;
        let deposit_amount = dec!(5);
        let deposit_transaction_id = 1;

        let mut accounts: HashMap<ClientId, Account> = HashMap::new();
        accounts.insert(
            client,
            Account {
                available: dec!(0),
                held: deposit_amount,
                frozen: false,
            },
        );

        let mut transactions: HashMap<TransactionId, Transaction> = HashMap::new();
        transactions.insert(
            deposit_transaction_id,
            Transaction::Deposit(TransactionData {
                client,
                transaction: deposit_transaction_id,
                amount: Some(deposit_amount),
            }),
        );

        let data = TransactionData {
            client,
            transaction: deposit_transaction_id,
            amount: None,
        };

        let res = resolve(&data, &mut accounts, &transactions);
        assert!(res.is_ok());

        let account = accounts.get(&client).unwrap();
        assert_eq!(account.available, deposit_amount);
    }

    #[test]
    fn cannot_resolve_unknown_transaction() {
        let client = 1;

        let mut accounts: HashMap<ClientId, Account> = HashMap::new();
        accounts.insert(
            client,
            Account {
                available: dec!(0),
                held: dec!(0),
                frozen: false,
            },
        );

        let transactions: HashMap<TransactionId, Transaction> = HashMap::new();

        let data = TransactionData {
            client,
            transaction: 1,
            amount: None,
        };

        let res = resolve(&data, &mut accounts, &transactions);
        assert!(res.is_err());
    }

    #[test]
    fn cannot_resolve_non_deposit() {
        let client = 1;
        let withdrawal_amount = dec!(5);
        let withdrawal_transaction_id = 1;

        let mut accounts: HashMap<ClientId, Account> = HashMap::new();
        accounts.insert(
            client,
            Account {
                available: withdrawal_amount,
                held: dec!(0),
                frozen: false,
            },
        );

        let mut transactions: HashMap<TransactionId, Transaction> = HashMap::new();
        transactions.insert(
            withdrawal_transaction_id,
            Transaction::Withdrawal(TransactionData {
                client,
                transaction: withdrawal_transaction_id,
                amount: Some(withdrawal_amount),
            }),
        );

        let data = TransactionData {
            client,
            transaction: 1,
            amount: None,
        };

        let res = resolve(&data, &mut accounts, &transactions);
        assert!(res.is_err());
    }
}
