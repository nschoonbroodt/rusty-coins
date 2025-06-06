#[derive(thiserror::Error, Debug, PartialEq)]
pub enum CoinsCoreError {
    #[error("Commodity with symbol '{0}' already exists")]
    CommodityAlreadyExists(String),
    #[error(
        "Invalid account name: {0}. Account names must starts with 'Assets:', 'Liabilities:', 'Income:', 'Expenses:', or 'Equity:'."
    )]
    InvalidAccountName(String),

    #[error(transparent)]
    Rusqlite(#[from] rusqlite::Error),
    #[error(transparent)]
    RusqliteMigration(#[from] rusqlite_migration::Error),
}
