use clap::Parser;
use coins_core::model::commodity::Commodity;

#[derive(Parser, Debug)]
pub struct CommodityOpt {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    List,
    Add { name: String, symbol: String },
}

pub fn run(model: &coins_core::CoinsModel, commodity_opt: CommodityOpt) -> anyhow::Result<()> {
    match &commodity_opt.command {
        Command::List => list(model)?,
        Command::Add { name, symbol } => add(model, name.clone(), symbol.clone())?,
    }
    Ok(())
}

fn list(model: &coins_core::CoinsModel) -> anyhow::Result<()> {
    let commodities = Commodity::all(model)?;
    commodities.iter().for_each(|commodity| {
        println!(
            "Commodity Name: {}, Symbol: {}",
            commodity.name(),
            commodity.symbol(),
        );
    });
    Ok(())
}

fn add(model: &coins_core::CoinsModel, name: String, symbol: String) -> anyhow::Result<()> {
    let commodity = Commodity::builder(model)
        .name(name)
        .symbol(symbol)
        .build()?;
    println!("Added commodity with ID: {}", commodity.id());
    Ok(())
}
