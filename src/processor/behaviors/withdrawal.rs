use crate::processor::{Account, ClientId, TransactionData};
use anyhow::{anyhow, Result};
use rust_decimal_macros::*;
use std::collections::HashMap;

pub fn withdrawal(data: &TransactionData, accounts: &mut HashMap<ClientId, Account>) -> Result<()> {
    let TransactionData { client, amount, .. } = data;
    let amount = amount.ok_or(anyhow!("Withdrawal should have the amount"))?;

    accounts
        .entry(*client)
        .and_modify(|account| {
            let not_frozen = !account.frozen;
            let has_funds = account.available >= amount;

            if not_frozen && has_funds {
                account.available -= amount
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
        assert!(res.is_ok());

        let account = accounts.get(&client);
        assert!(account.is_some());

        let account = account.unwrap();
        assert_eq!(account.available, available);
    }
}
