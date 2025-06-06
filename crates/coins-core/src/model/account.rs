use core::fmt;

use crate::prelude::*;
use bon::bon;

#[derive(Debug)]
pub struct AccountName(String);
impl AccountName {
    pub fn new(name: &str) -> Result<Self> {
        if !name.starts_with("Assets:")
            && !name.starts_with("Liabilities:")
            && !name.starts_with("Revenue:")
            && !name.starts_with("Expenses:")
            && !name.starts_with("Equity:")
        {
            return Err(CoinsCoreError::InvalidAccountName(name.to_string()));
        }
        Ok(Self(name.to_string()))
    }
}
impl fmt::Display for AccountName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct Account {
    id: i64,
    name: AccountName,
}

#[bon]
impl Account {
    #[builder]
    pub fn new(#[builder(start_fn)] model: &super::CoinsModel, name: AccountName) -> Result<Self> {
        let mut stmt = model
            .conn
            .prepare("INSERT INTO accounts (name) VALUES (?1) RETURNING id, name")?;
        let mut account_rows = stmt.query(rusqlite::params![name.0])?;
        let row = account_rows.next()?.unwrap();
        Ok(Self {
            id: row.get(0)?,
            name: AccountName(row.get(1)?),
        })
    }

    pub fn delete(self, model: &super::CoinsModel) -> Result<()> {
        model.conn.execute(
            "DELETE FROM accounts WHERE id = ?1",
            rusqlite::params![self.id],
        )?;
        Ok(())
    }

    pub fn by_id(model: &super::CoinsModel, id: i64) -> Result<Option<Self>> {
        let mut stmt = model
            .conn
            .prepare("SELECT id, name FROM accounts WHERE id = ?1")?;
        let mut rows = stmt.query(rusqlite::params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(Self {
                id: row.get(0)?,
                name: AccountName(row.get(1)?),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn name(&self) -> &AccountName {
        &self.name
    }

    pub fn all(model: &super::CoinsModel) -> Result<Vec<Self>> {
        let mut stmt = model.conn.prepare("SELECT id, name FROM accounts")?;
        let accounts = stmt
            .query_map([], |row| {
                Ok(Account {
                    id: row.get(0)?,
                    name: AccountName(row.get(1)?),
                })
            })?
            .flatten()
            .collect();

        Ok(accounts)
    }
}

#[cfg(test)]
mod tests {
    use super::super::CoinsModel;
    use super::*;

    #[test]
    fn test_accounts() {
        let model = CoinsModel::new(None).unwrap();
        let conn = &model.conn;
        let accounts_pre = Account::all(&model).unwrap();

        conn.execute(r#"INSERT INTO accounts (name) VALUES ("Account1")"#, ())
            .unwrap();

        println!("{}", pretty_sqlite::pretty_table(conn, "accounts").unwrap());

        let accounts = Account::all(&model).unwrap();
        assert_eq!(accounts.len(), accounts_pre.len() + 1);
        assert!(accounts.iter().any(|a| a.name().0 == "Account1"));
    }

    #[test]
    fn test_create_account() {
        let model = CoinsModel::new(None).unwrap();
        let account = Account::builder(&model)
            .name(AccountName::new("Assets:Test Account").unwrap())
            .build()
            .unwrap();

        println!(
            "{}",
            pretty_sqlite::pretty_table(&model.conn, "accounts").unwrap()
        );

        assert_eq!(account.name().0, "Assets:Test Account");
        assert!(account.id() > 0);

        let accounts = Account::all(&model).unwrap();
        assert!(accounts.iter().any(|a| a.id() == account.id()));
    }

    #[test]
    fn test_delete_account() {
        let model = CoinsModel::new(None).unwrap();
        let account = Account::builder(&model)
            .name(AccountName::new("Assets::Account to Delete").unwrap())
            .build()
            .unwrap();

        println!(
            "{}",
            pretty_sqlite::pretty_table(&model.conn, "accounts").unwrap()
        );

        let account_id = account.id();

        account.delete(&model).unwrap();

        let accounts = Account::all(&model).unwrap();
        assert!(!accounts.iter().any(|a| a.id() == account_id));
    }

    #[test]
    fn test_get_account_by_id() {
        let model = CoinsModel::new(None).unwrap();
        let account = Account::builder(&model)
            .name(AccountName::new("Account to Retrieve").unwrap())
            .build()
            .unwrap();

        println!(
            "{}",
            pretty_sqlite::pretty_table(&model.conn, "accounts").unwrap()
        );

        let retrieved_account = Account::by_id(&model, account.id()).unwrap();
        assert!(retrieved_account.is_some());
        assert_eq!(retrieved_account.unwrap().name().0, "Account to Retrieve");
    }
}
