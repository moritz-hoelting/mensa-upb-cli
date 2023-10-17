use clap::Parser;
use futures::future::join_all;
use mensa_upb_cli::{cli_args::PriceLevel, menu_table, Mensa};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if !cli.mensa.is_empty() {
        let mensa = cli.mensa;
        let menu = join_all(mensa.iter().map(|m| m.get_menu()))
            .await
            .into_iter()
            .filter_map(|menu| menu.ok())
            .collect::<Vec<_>>();
        let table = menu_table(&menu, cli.price_level, mensa.len() > 1);
        println!("{}", table);
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Mensa auswählen
    #[arg(short, long, value_enum, default_values_t = [Mensa::Forum, Mensa::Academica])]
    mensa: Vec<Mensa>,
    /// Preisstufe auswählen
    #[arg(short, long)]
    price_level: Option<PriceLevel>,
}
