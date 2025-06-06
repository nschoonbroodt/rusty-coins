use crate::prelude::*;
use bon::bon;

#[derive(Debug)]
pub struct Account {
    id: i64,
    name: String,
}

#[bon]
impl Account {
    #[builder]
    pub fn new(#[builder(start_fn)] model: &super::CoinsModel, name: String) -> Result<Self> {
        let mut stmt = model
            .conn
            .prepare("INSERT INTO accounts (name) VALUES (?1) RETURNING id, name")?;
        let mut account_rows = stmt.query(rusqlite::params![name])?;
        let row = account_rows.next()?.unwrap();
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }

    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn all(model: &super::CoinsModel) -> Result<Vec<Self>> {
        let mut stmt = model.conn.prepare("SELECT id, name FROM accounts")?;
        let accounts = stmt
            .query_map([], |row| {
                Ok(Account {
                    id: row.get(0)?,
                    name: row.get(1)?,
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

        conn.execute(r#"INSERT INTO accounts (name) VALUES ("Account1")"#, ())
            .unwrap();

        println!("{}", pretty_sqlite::pretty_table(conn, "accounts").unwrap());

        let accounts = Account::all(&model).unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].name(), "Account1");
    }

    #[test]
    fn test_create_account() {
        let model = CoinsModel::new(None).unwrap();
        let account = Account::builder(&model)
            .name("Test Account".to_string())
            .build()
            .unwrap();

        println!(
            "{}",
            pretty_sqlite::pretty_table(&model.conn, "accounts").unwrap()
        );

        assert_eq!(account.name(), "Test Account");
        assert!(account.id() > 0);

        let accounts = Account::all(&model).unwrap();
        assert!(accounts.iter().any(|a| a.id() == account.id()));
    }
}
