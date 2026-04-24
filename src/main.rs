use chrono::{Days, Utc};
use clap::Parser;
use mensa_upb_cli::{
    cli_args::{Filter, PriceLevel},
    generate_json, menu_table,
    util::all_menus,
    Canteen,
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if !cli.mensa.is_empty() {
        let mensen = cli.mensa;
        let menu = all_menus(
            &mensen,
            cli.days_ahead
                .map(|days_ahead| (Utc::now() + Days::new(days_ahead)).date_naive()),
            &cli.filter,
        )
        .await;

        match cli.output_format {
            OutputFormat::Table => {
                let table = menu_table(&menu, cli.price_level, mensen.len() > 1);
                println!("{}", table);
            }
            OutputFormat::Json => {
                let output = generate_json(&menu);
                println!("{output}");
            }
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Mensa auswählen
    #[arg(short, long, value_enum, default_values_t = [Canteen::Forum, Canteen::Academica])]
    mensa: Vec<Canteen>,
    /// Preisstufe auswählen
    #[arg(short, long)]
    price_level: Option<PriceLevel>,
    /// Nächsten Tage anzeigen
    #[arg(short, long)]
    days_ahead: Option<u64>,
    /// Nur Gerichte anzeigen, die vegan oder vegetarisch sind
    #[arg(short, long)]
    filter: Vec<Filter>,

    #[arg(short, long, value_enum, default_value_t = OutputFormat::Table)]
    output_format: OutputFormat,
}

#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
enum OutputFormat {
    #[default]
    Table,
    Json,
}
