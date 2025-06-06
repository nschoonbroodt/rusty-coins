pub use crate::opt::Opt;

mod account;
mod commodity;
mod opt;

pub fn run(opt: Opt) -> anyhow::Result<()> {
    let model = coins_core::CoinsModel::new(opt.database.as_deref())?;

    match opt.command {
        opt::Command::Account(account_opt) => account::run(&model, account_opt)?,
        opt::Command::Commodity(commodity_opt) => commodity::run(&model, commodity_opt)?,
    }

    Ok(())
}
