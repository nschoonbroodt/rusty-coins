pub mod account;
pub mod commodity;
mod migrations;

use crate::prelude::*;

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
        migrations::apply_migrations(&mut conn)?;

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
