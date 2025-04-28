pub mod cli_args;
mod daily_menu;
mod dish;
mod mensa;
mod menu_table;
pub mod util;
mod json;

pub use daily_menu::DailyMenu;
pub use dish::Dish;
pub use mensa::Mensa;
pub use menu_table::menu_table;
pub use json::generate_json;
