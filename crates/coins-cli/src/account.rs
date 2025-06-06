use clap::Parser;

#[derive(Parser, Debug)]
pub struct AccountOpt {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    List,
    Add,
}

pub fn run(account_opt: AccountOpt) -> anyhow::Result<()> {
    match &account_opt.command {
        Command::List => todo!(),
        Command::Add => todo!(),
    }
    Ok(())
}
