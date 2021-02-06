use crate::processor::{Account, Accounts, TransactionData};
use anyhow::Result;
use rust_decimal_macros::*;

pub fn deposit(data: &TransactionData, accounts: &mut Accounts) -> Result<()> {
    let TransactionData { client, amount, .. } = data;
    let amount = amount.expect("Deposit should have the amount");

    accounts
        .entry(*client)
        .and_modify(|account| {
            if !account.frozen {
                account.available += amount
            }
        })
        .or_insert_with(|| Account {
            available: amount,
            held: dec!(0),
            frozen: false,
        });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn deposit_works() {
        let client = 1;
        let amount = dec!(5);

        let mut accounts: Accounts = HashMap::new();

        let data = TransactionData {
            client,
            transaction: 1,
            amount: Some(amount),
        };

        let res = deposit(&data, &mut accounts);
        assert!(res.is_ok());

        let account = accounts.get(&client);
        assert!(account.is_some());

        let account = account.unwrap();
        assert_eq!(account.available, amount);
    }

    #[test]
    fn cannot_deposit_into_locked_account() {
        let client = 1;
        let amount = dec!(5);
        let available = dec!(0);

        let mut accounts: Accounts = HashMap::new();
        accounts.insert(
            client,
            Account {
                available,
                held: dec!(0),
                frozen: true,
            },
        );

        let data = TransactionData {
            client,
            transaction: 1,
            amount: Some(amount),
        };

        let res = deposit(&data, &mut accounts);
        assert!(res.is_ok());

        let account = accounts.get(&client);
        assert!(account.is_some());

        let account = account.unwrap();
        assert_eq!(account.available, available);
    }
}
