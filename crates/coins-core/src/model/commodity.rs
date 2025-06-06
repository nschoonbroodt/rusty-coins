use crate::prelude::*;
use bon::bon;

#[derive(Debug)]
pub struct Commodity {
    id: i64,
    name: String,
    symbol: String,
}

#[bon]
impl Commodity {
    #[builder]
    pub fn new(
        #[builder(start_fn)] model: &super::CoinsModel,
        name: String,
        symbol: String,
    ) -> Result<Self> {
        let mut stmt = model.conn.prepare(
            "INSERT INTO commodities (name, symbol) VALUES (?1, ?2) RETURNING id, name, symbol",
        )?;

        let mut commodity_rows = stmt.query(rusqlite::params![name, symbol])?;

        let row = commodity_rows
            .next()
            .map_err(|e| {
                if e.to_string()
                    .contains("UNIQUE constraint failed: commodities.symbol")
                {
                    CoinsCoreError::CommodityAlreadyExists(symbol.clone())
                } else {
                    e.into()
                }
            })?
            .unwrap();

        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            symbol: row.get(2)?,
        })
    }
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn all(model: &super::CoinsModel) -> Result<Vec<Self>> {
        let mut stmt = model
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
    use super::*;

    #[test]
    fn test_commodities() {
        let model = super::super::CoinsModel::new(None).unwrap();
        let conn = &model.conn;

        conn.execute(
            r#"INSERT INTO commodities (name, symbol) VALUES ("Euro", "EUR"), ("US Dollar", "USD")"#,
            (),
        )
        .unwrap();

        println!(
            "{}",
            pretty_sqlite::pretty_table(conn, "commodities").unwrap()
        );

        let commodities = Commodity::all(&model).unwrap();
        assert_eq!(commodities.len(), 2);
        assert_eq!(commodities[0].name(), "Euro");
        assert_eq!(commodities[0].symbol(), "EUR");
        assert_eq!(commodities[1].name(), "US Dollar");
        assert_eq!(commodities[1].symbol(), "USD");
    }

    #[test]
    fn test_new() {
        let model = super::super::CoinsModel::new(None).unwrap();
        let com: Commodity = Commodity::builder(&model)
            .name("Euro".to_string())
            .symbol("EUR".to_string())
            .build()
            .unwrap();

        println!(
            "{}",
            pretty_sqlite::pretty_table(&model.conn, "commodities").unwrap()
        );

        assert_eq!(com.name(), "Euro");
        assert_eq!(com.symbol(), "EUR");

        let commodities = Commodity::all(&model).unwrap();
        assert!(
            commodities
                .iter()
                .any(|a| a.id() == com.id() && a.name() == "Euro" && a.symbol() == "EUR")
        );
    }
    #[test]
    fn test_new_duplicate_symbol() {
        let model = super::super::CoinsModel::new(None).unwrap();
        let _: Commodity = Commodity::builder(&model)
            .name("Euro".to_string())
            .symbol("EUR".to_string())
            .build()
            .unwrap();
        let res = Commodity::builder(&model)
            .name("Euro".to_string())
            .symbol("EUR".to_string())
            .build();

        let x = res.unwrap_err();
        assert_eq!(x, CoinsCoreError::CommodityAlreadyExists("EUR".to_string()));
    }
}
