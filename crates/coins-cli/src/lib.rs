pub use crate::opt::Opt;

mod account;
mod commodity;
mod opt;

pub fn run(opt: Opt) -> anyhow::Result<()> {
    match opt.command {
        opt::Command::Account(account_opt) => account::run(account_opt)?,
        opt::Command::Commodity(commodity_opt) => commodity::run(commodity_opt)?,
    }

    Ok(())
}
