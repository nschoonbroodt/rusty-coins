fn main() {
    let model = coins_core::model::CoinsModel::new(None).unwrap();

    let _ = pretty_sqlite::print_table(&model.conn, "accounts");
}
