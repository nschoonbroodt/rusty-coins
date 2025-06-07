use clap::Parser;
use coins_core::model::account::{Account, AccountName};

#[derive(Parser, Debug)]
pub struct AccountOpt {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    List,
    Add {
        name: String,
        opening_date: Option<chrono::NaiveDate>,
    },
    Delete {
        id: i64,
    },
}

pub fn run(model: &coins_core::CoinsModel, account_opt: AccountOpt) -> anyhow::Result<()> {
    match &account_opt.command {
        Command::List => list(model)?,
        Command::Add { name, opening_date } => add(model, name, opening_date)?,
        Command::Delete { id } => delete(model, *id)?,
    }
    Ok(())
}

fn list(model: &coins_core::CoinsModel) -> anyhow::Result<()> {
    let accounts = Account::all(model)?;
    accounts.iter().for_each(|account| {
        println!(
            "Account ID: {}, Name: {}, Opening date: {:?}",
            account.id(),
            account.name(),
            account.opening_date(),
        );
    });
    Ok(())
}

fn add(
    model: &coins_core::CoinsModel,
    name: &str,
    opening_date: &Option<chrono::NaiveDate>,
) -> anyhow::Result<()> {
    let name = AccountName::new(name).unwrap();
    let account = Account::builder(model)
        .name(name)
        .maybe_opening_date(*opening_date)
        .build()?;
    println!("Added account with ID: {}", account.id());
    Ok(())
}

fn delete(model: &coins_core::CoinsModel, id: i64) -> anyhow::Result<()> {
    let account = Account::by_id(model, id)?;
    if let Some(account) = account {
        println!("Deleting account with ID: {}", account.id());
        account.delete(model)?;
    } else {
        println!("No account found with ID: {}", id);
    }
    Ok(())
}
