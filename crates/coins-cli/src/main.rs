use clap::Parser;

fn main() {
    let _ = dotenvy::dotenv();
    let opt = coins_cli::Opt::parse();
    if let Err(error) = coins_cli::run(opt) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
