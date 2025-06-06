use clap::Parser;

#[derive(Parser, Debug)]
pub struct CommodityOpt {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    List,
    Add,
}

pub fn run(commodity_opt: CommodityOpt) -> anyhow::Result<()> {
    match &commodity_opt.command {
        Command::List => todo!(),
        Command::Add => todo!(),
    }
    Ok(())
}
