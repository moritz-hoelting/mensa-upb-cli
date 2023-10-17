use chrono::NaiveDate;
use futures::future::join_all;

use crate::{DailyMenu, Mensa};

pub async fn all_menus(mensen: &[Mensa], day: Option<NaiveDate>) -> Vec<DailyMenu> {
    join_all(mensen.iter().map(|m| m.get_menu(day)))
        .await
        .into_iter()
        .filter_map(|menu| menu.ok())
        .collect::<Vec<_>>()
}
