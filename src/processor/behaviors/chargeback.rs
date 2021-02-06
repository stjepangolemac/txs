use crate::processor::{Accounts, Transaction, TransactionData, Transactions};
use anyhow::{anyhow, Result};

pub fn chargeback(
    data: &TransactionData,
    accounts: &mut Accounts,
    transactions: &mut Transactions,
) -> Result<()> {
    let TransactionData {
        client,
        transaction,
        ..
    } = data;

    if let Some((referenced_transaction, referenced_transaction_disputed)) =
        transactions.get_mut(transaction)
    {
        if !*referenced_transaction_disputed {
            return Ok(());
        }

        let amount = match referenced_transaction {
            Transaction::Deposit(data) => data
                .amount
                .ok_or_else(|| anyhow!("Referenced deposit should have the amount")),
            _ => Err(anyhow!("Cannot chargeback non deposit")),
        }?;

        let mut res = Ok(());

        accounts.entry(*client).and_modify(|account| {
            if account.held < amount {
                res = Err(anyhow!("Cannot chargeback, insufficient held funds"));
                return;
            }

            account.held -= amount;
            account.frozen = true;
        });

        return res;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processor::Account;
    use rust_decimal_macros::*;
    use std::collections::HashMap;

    #[test]
    fn chargeback_works() {
        let client = 1;
        let deposit_amount = dec!(5);
        let deposit_transaction_id = 1;

        let mut accounts: Accounts = HashMap::new();
        accounts.insert(
            client,
            Account {
                available: dec!(0),
                held: deposit_amount,
                frozen: false,
            },
        );

        let mut transactions: Transactions = HashMap::new();
        transactions.insert(
            deposit_transaction_id,
            (
                Transaction::Deposit(TransactionData {
                    client,
                    transaction: deposit_transaction_id,
                    amount: Some(deposit_amount),
                }),
                true,
            ),
        );

        let data = TransactionData {
            client,
            transaction: deposit_transaction_id,
            amount: None,
        };

        let res = chargeback(&data, &mut accounts, &mut transactions);
        assert!(res.is_ok());

        let account = accounts.get(&client).unwrap();
        assert_eq!(account.available, dec!(0));
        assert_eq!(account.held, dec!(0));
    }

    #[test]
    fn cannot_chargeback_non_deposit() {
        let client = 1;
        let withdrawal_amount = dec!(5);
        let withdrawal_transaction_id = 1;

        let mut accounts: Accounts = HashMap::new();
        accounts.insert(
            client,
            Account {
                available: dec!(0),
                held: withdrawal_amount,
                frozen: false,
            },
        );

        let mut transactions: Transactions = HashMap::new();
        transactions.insert(
            withdrawal_transaction_id,
            (
                Transaction::Withdrawal(TransactionData {
                    client,
                    transaction: withdrawal_transaction_id,
                    amount: Some(withdrawal_amount),
                }),
                true,
            ),
        );

        let data = TransactionData {
            client,
            transaction: withdrawal_transaction_id,
            amount: None,
        };

        let res = chargeback(&data, &mut accounts, &mut transactions);
        assert!(res.is_err());
    }

    #[test]
    fn cannot_chargeback_no_held_funds() {
        let client = 1;
        let deposit_amount = dec!(5);
        let deposit_transaction_id = 1;

        let mut accounts: Accounts = HashMap::new();
        accounts.insert(
            client,
            Account {
                available: dec!(0),
                held: deposit_amount - dec!(1),
                frozen: false,
            },
        );

        let mut transactions: Transactions = HashMap::new();
        transactions.insert(
            deposit_transaction_id,
            (
                Transaction::Deposit(TransactionData {
                    client,
                    transaction: deposit_transaction_id,
                    amount: Some(deposit_amount),
                }),
                true,
            ),
        );

        let data = TransactionData {
            client,
            transaction: deposit_transaction_id,
            amount: None,
        };

        let res = chargeback(&data, &mut accounts, &mut transactions);
        assert!(res.is_err());
    }
}
