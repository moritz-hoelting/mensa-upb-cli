use chrono::NaiveDate;
use futures::future::join_all;
use rust_decimal::Decimal;

use crate::{Canteen, DailyMenu};

pub async fn all_menus(canteens: &[Canteen], day: Option<NaiveDate>) -> Vec<DailyMenu> {
    join_all(canteens.iter().map(|m| m.get_menu(day)))
        .await
        .into_iter()
        .filter_map(|menu| menu.ok())
        .collect::<Vec<_>>()
}

pub fn normalize_price_bigdecimal(price: Decimal) -> Decimal {
    price.normalize().round_dp(2)
}

pub fn first_non_empty_string(strings: impl IntoIterator<Item = String>) -> Option<String> {
    strings.into_iter().find(|s| !s.trim().is_empty())
}
