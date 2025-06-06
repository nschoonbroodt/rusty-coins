use coins_core::model::commodity::Commodity;

fn main() {
    let model = coins_core::CoinsModel::new(None).unwrap();
    let com: Commodity = Commodity::builder(model)
        .name("Euro".to_string())
        .symbol("EUR".to_string())
        .build();

    println!("Commodity: {:?}", com);
}
