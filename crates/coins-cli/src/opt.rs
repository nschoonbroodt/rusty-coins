use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(version, about, author)]
pub struct Opt {
    #[clap(long, short = 'D', env)]
    pub database: Option<PathBuf>,
    #[clap(subcommand)]
    pub(crate) command: Command,
}

#[derive(Parser, Debug)]
pub(crate) enum Command {
    Account(crate::account::AccountOpt),
    Commodity(crate::commodity::CommodityOpt),
}
