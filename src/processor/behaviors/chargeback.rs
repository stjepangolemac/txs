use crate::processor::{
    Account, Accounts, ClientId, Transaction, TransactionData, TransactionId, Transactions,
};
use anyhow::{anyhow, Result};
use std::collections::HashMap;

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

        accounts.entry(*client).and_modify(|account| {
            let has_held_funds = account.held >= amount;

            if has_held_funds {
                account.held -= amount;
                account.frozen = true;
            }
        });

        return Ok(());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::*;

    #[test]
    fn chargeback_works() {
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

        let mut accounts: HashMap<ClientId, Account> = HashMap::new();
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
}
