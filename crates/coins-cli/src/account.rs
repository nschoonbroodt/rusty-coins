use clap::Parser;
use coins_core::model::account::Account;

#[derive(Parser, Debug)]
pub struct AccountOpt {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    List,
    Add { name: String },
}

pub fn run(model: &coins_core::CoinsModel, account_opt: AccountOpt) -> anyhow::Result<()> {
    match &account_opt.command {
        Command::List => list(model)?,
        Command::Add { name } => add(model, name.clone())?,
    }
    Ok(())
}

fn list(model: &coins_core::CoinsModel) -> anyhow::Result<()> {
    let accounts = Account::all(model)?;
    accounts.iter().for_each(|account| {
        println!("Account ID: {}, Name: {}", account.id(), account.name());
    });
    Ok(())
}

fn add(model: &coins_core::CoinsModel, name: String) -> anyhow::Result<()> {
    let account = Account::builder(model).name(name).build()?;
    println!("Added account with ID: {}", account.id());
    Ok(())
}
