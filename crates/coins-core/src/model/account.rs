use crate::prelude::*;

#[derive(Debug)]
pub struct Account {
    id: i64,
    name: String,
}

impl super::CoinsModel {
    pub fn accounts(&self) -> Result<Vec<Account>> {
        let mut stmt = self.conn.prepare("SELECT id, name FROM accounts")?;
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

    #[test]
    fn test_display_accounts() {
        let model = CoinsModel::new(None).unwrap();
        let conn = &model.conn;
        // TODO: movo this to a test
        conn.execute(r#"INSERT INTO accounts (name) VALUES ("Account1")"#, ())
            .unwrap();
        conn.execute(r#"UPDATE accounts SET name = "Account1_1""#, ())
            .unwrap();

        println!("{}", pretty_sqlite::pretty_table(conn, "accounts").unwrap());

        println!("{:?}", model.accounts().unwrap());
    }
}
