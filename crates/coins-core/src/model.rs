pub mod account;

use std::sync::LazyLock;

use crate::prelude::*;

use include_dir::{Dir, include_dir};
use rusqlite_migration::Migrations;

static MIGRATION_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");
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
}
