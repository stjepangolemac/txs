use crate::processor::{Account, ClientId, TransactionData};
use anyhow::{anyhow, Result};
use rust_decimal_macros::*;
use std::collections::HashMap;

pub fn withdrawal(data: &TransactionData, accounts: &mut HashMap<ClientId, Account>) -> Result<()> {
    let TransactionData { client, amount, .. } = data;
    let amount = amount
        .ok_or(anyhow!("Withdrawal should have the amount"))
        .and_then(|amount| {
            if amount < dec!(0) {
                Err(anyhow!("Deposit amount cannot be negative"))
            } else {
                Ok(amount)
            }
        })?;

    let mut res = Ok(());

    accounts.entry(*client).and_modify(|account| {
        if account.frozen {
            res = Err(anyhow!("Cannot withdraw from a frozen account"));
            return;
        }

        if account.available < amount {
            res = Err(anyhow!("Cannot withdraw, insufficient funds"));
            return;
        }

        account.available -= amount
    });

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::*;

    #[test]
    fn withdrawal_works() {
        let client = 1;
        let amount = dec!(3);
        let available = dec!(5);

        let mut accounts: HashMap<ClientId, Account> = HashMap::new();
        accounts.insert(
            client,
            Account {
                available,
                held: dec!(0),
                frozen: false,
            },
        );

        let data = TransactionData {
            client,
            transaction: 1,
            amount: Some(amount),
        };

        let res = withdrawal(&data, &mut accounts);
        assert!(res.is_ok());

        let account = accounts.get(&client);
        assert!(account.is_some());

        let account = account.unwrap();
        assert_eq!(account.available, available - amount);
    }

    #[test]
    fn cannot_withdraw_without_amount() {
        let client = 1;

        let mut accounts: HashMap<ClientId, Account> = HashMap::new();
        accounts.insert(
            client,
            Account {
                available: dec!(0),
                held: dec!(0),
                frozen: true,
            },
        );

        let data = TransactionData {
            client,
            transaction: 1,
            amount: None,
        };

        let res = withdrawal(&data, &mut accounts);
        assert!(res.is_err());
    }

    #[test]
    fn withdraw_amount_must_be_positive() {
        let client = 1;

        let mut accounts: HashMap<ClientId, Account> = HashMap::new();
        accounts.insert(
            client,
            Account {
                available: dec!(10),
                held: dec!(0),
                frozen: true,
            },
        );

        let data = TransactionData {
            client,
            transaction: 1,
            amount: Some(dec!(-1)),
        };

        let res = withdrawal(&data, &mut accounts);
        assert!(res.is_err());
    }

    #[test]
    fn cannot_withdraw_from_locked_account() {
        let client = 1;
        let amount = dec!(3);
        let available = dec!(5);

        let mut accounts: HashMap<ClientId, Account> = HashMap::new();
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

        let res = withdrawal(&data, &mut accounts);
        assert!(res.is_err());
    }

    #[test]
    fn cannot_withdraw_if_no_funds() {
        let client = 1;
        let amount = dec!(4);
        let available = dec!(2);

        let mut accounts: HashMap<ClientId, Account> = HashMap::new();
        accounts.insert(
            client,
            Account {
                available,
                held: dec!(0),
                frozen: false,
            },
        );

        let data = TransactionData {
            client,
            transaction: 1,
            amount: Some(amount),
        };

        let res = withdrawal(&data, &mut accounts);
        assert!(res.is_err());
    }
}
