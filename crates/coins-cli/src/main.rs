use coins_core::model::{account::Account, commodity::Commodity};

fn main() {
    let model = coins_core::CoinsModel::new(None).unwrap();
    let com: Commodity = Commodity::builder(&model)
        .name("Euro".to_string())
        .symbol("EUR".to_string())
        .build()
        .unwrap();
    println!("New commodity: {:?}", com);

    let account: Account = Account::builder(&model)
        .name("My account".to_string())
        .build()
        .unwrap();
    println!("New account: {:?}", account);

    println!("All commodities: {:?}", Commodity::all(&model).unwrap());
    println!("All accounts: {:?}", Account::all(&model).unwrap());
}
