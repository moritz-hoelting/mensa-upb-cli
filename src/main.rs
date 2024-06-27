use std::io;

use mensa_upb_cli::{app::App, tui};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = tui::restore();
        panic_hook(panic_info);
    }));
    let app_result = App::default().run(&mut terminal).await;
    tui::restore()?;
    app_result
}
