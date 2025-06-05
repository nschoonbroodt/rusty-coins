use crate::prelude::*;

#[derive(Debug)]
pub struct Commodity {
    id: i64,
    name: String,
    symbol: String,
}

impl Commodity {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

impl super::CoinsModel {
    pub fn commodities(&self) -> Result<Vec<Commodity>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, symbol FROM commodities")?;
        let commodities = stmt
            .query_map([], |row| {
                Ok(Commodity {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    symbol: row.get(2)?,
                })
            })?
            .flatten()
            .collect();

        Ok(commodities)
    }
}

#[cfg(test)]
mod tests {
    use super::super::CoinsModel;

    #[test]
    fn test_commodities() {
        let model = CoinsModel::new(None).unwrap();
        let conn = &model.conn;

        conn.execute(
            r#"INSERT INTO commodities (name, symbol) VALUES ("Euro", "EUR")"#,
            (),
        )
        .unwrap();

        println!(
            "{}",
            pretty_sqlite::pretty_table(conn, "commodities").unwrap()
        );

        let commodities = model.commodities().unwrap();
        assert_eq!(commodities.len(), 1);
        assert_eq!(commodities[0].name(), "Euro");
        assert_eq!(commodities[0].symbol(), "EUR");
    }
}
