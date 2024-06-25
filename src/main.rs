use std::io;

use mensa_upb_cli::{app::App, tui};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
