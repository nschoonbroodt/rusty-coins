#[derive(thiserror::Error, Debug)]
pub enum CoinsCoreError {
    #[error(transparent)]
    Rusqlite(#[from] rusqlite::Error),
    #[error(transparent)]
    RusqliteMigration(#[from] rusqlite_migration::Error),
}
