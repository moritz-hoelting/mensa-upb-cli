use std::fmt::Display;

use chrono::NaiveDate;
use clap::ValueEnum;

use crate::{cli_args::Filter, DailyMenu};

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum, serde::Serialize)]
pub enum Canteen {
    Forum,
    Academica,
    GrillCafe,
    ZM2,
    Basilica,
    Atrium,
}

impl Canteen {
    pub async fn get_menu(
        &self,
        day: Option<NaiveDate>,
        filters: &[Filter],
    ) -> Result<DailyMenu, reqwest::Error> {
        let date = day.unwrap_or_else(|| chrono::Local::now().naive_local().date());
        DailyMenu::scrape(&date, &date, *self, filters).await
    }

    pub fn get_venue_id(&self) -> &'static str {
        match self {
            Self::Academica => "mensa",
            Self::Forum => "mensa-forum",
            Self::ZM2 => "mensa-zm2",
            Self::Basilica => "mensa-hamm",
            Self::Atrium => "mensa-lippstadt",
            Self::GrillCafe => "grill-cafe",
        }
    }
}

impl Display for Canteen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Forum => "Forum",
            Self::Academica => "Academica",
            Self::GrillCafe => "Grill | Café",
            Self::ZM2 => "ZM2",
            Self::Basilica => "Basilica",
            Self::Atrium => "Atrium",
        })
    }
}
