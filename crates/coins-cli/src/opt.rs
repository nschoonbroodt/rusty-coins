use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, author)]
pub struct Opt {
    #[clap(subcommand)]
    pub(crate) command: Command,
}

#[derive(Parser, Debug)]
pub(crate) enum Command {
    Account(crate::account::AccountOpt),
    Commodity(crate::commodity::CommodityOpt),
}
