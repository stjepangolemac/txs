use crate::processor::{Account, ClientId, TransactionData};
use rust_decimal_macros::*;
use std::collections::HashMap;

pub fn deposit(data: &TransactionData, accounts: &mut HashMap<ClientId, Account>) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_works() {
        let client = 1;
        let amount = dec!(5);

        let mut accounts: HashMap<ClientId, Account> = HashMap::new();

        let data = TransactionData {
            client,
            transaction: 1,
            amount: Some(amount),
        };

        deposit(&data, &mut accounts);

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

        deposit(&data, &mut accounts);

        let account = accounts.get(&client);
        assert!(account.is_some());

        let account = account.unwrap();
        assert_eq!(account.available, available);
    }
}
