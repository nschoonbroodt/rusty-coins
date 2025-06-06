#[derive(thiserror::Error, Debug, PartialEq)]
pub enum CoinsCoreError {
    #[error("Commodity with symbol '{0}' already exists")]
    CommodityAlreadyExists(String),
    #[error(transparent)]
    Rusqlite(#[from] rusqlite::Error),
    #[error(transparent)]
    RusqliteMigration(#[from] rusqlite_migration::Error),
}
