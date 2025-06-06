use crate::prelude::*;
use std::sync::LazyLock;

use include_dir::{Dir, include_dir};
use rusqlite_migration::Migrations;

static MIGRATION_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");
// Define migrations. These are applied atomically.
static MIGRATIONS: LazyLock<Migrations<'static>> =
    LazyLock::new(|| Migrations::from_directory(&MIGRATION_DIR).unwrap());

pub(super) fn apply_migrations(conn: &mut rusqlite::Connection) -> Result<()> {
    MIGRATIONS.to_latest(conn)?;
    Ok(())
}
