use std::sync::LazyLock;

use crate::prelude::*;

use include_dir::{Dir, include_dir};
use rusqlite_migration::Migrations;

static MIGRATION_DIR: Dir = include_dir!("crates/coins-core/migrations");
// Define migrations. These are applied atomically.
static MIGRATIONS: LazyLock<Migrations<'static>> =
    LazyLock::new(|| Migrations::from_directory(&MIGRATION_DIR).unwrap());

pub struct CoinsModel {
    pub conn: rusqlite::Connection, // TODO: remove pub
}

impl CoinsModel {
    pub fn new(path: Option<&std::path::Path>) -> Result<Self> {
        let mut conn = if let Some(path) = path {
            rusqlite::Connection::open(path)?
        } else {
            rusqlite::Connection::open_in_memory()?
        };
        MIGRATIONS.to_latest(&mut conn).unwrap();

        Ok(Self { conn })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _model = CoinsModel::new(None).unwrap();
    }

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
    }
}
