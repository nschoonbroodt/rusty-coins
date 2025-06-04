#[derive(thiserror::Error, Debug)]
pub enum CoinsCoreError {
    #[error(transparent)]
    Rusqlite(#[from] rusqlite::Error),
}
