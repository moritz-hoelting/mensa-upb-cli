use clap::ValueEnum;

use crate::{DailyMenu, Dish};

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum Mensa {
    Forum,
    Academica,
    Picknick,
    BonaVista,
    GrillCafe,
    ZM2,
    Basilica,
    Atrium,
}

impl Mensa {
    pub fn get_url(&self) -> &str {
        match self {
            Self::Forum => "https://www.studierendenwerk-pb.de/gastronomie/speiseplaene/forum/",
            Self::Academica => "https://www.studierendenwerk-pb.de/gastronomie/speiseplaene/mensa-academica/",
            Self::Picknick => "https://www.studierendenwerk-pb.de/gastronomie/speiseplaene/picknick/",
            Self::BonaVista => "https://www.studierendenwerk-pb.de/gastronomie/speiseplaene/bona-vista/",
            Self::GrillCafe => "https://www.studierendenwerk-pb.de/gastronomie/speiseplaene/grillcafe/",
            Self::ZM2 => "https://www.studierendenwerk-pb.de/gastronomie/speiseplaene/mensa-zm2/",
            Self::Basilica => "https://www.studierendenwerk-pb.de/gastronomie/speiseplaene/mensa-basilica-hamm/",
            Self::Atrium => "https://www.studierendenwerk-pb.de/gastronomie/speiseplaene/mensa-atrium-lippstadt/",
        }
    }

    pub async fn get_menu(&self) -> Result<DailyMenu, reqwest::Error> {
        let (main_dishes, side_dishes, desserts) = scrape_menu(self.get_url()).await?;
        Ok(DailyMenu::new(*self, main_dishes, side_dishes, desserts))
    }
}

impl ToString for Mensa {
    fn to_string(&self) -> String {
        match self {
            Self::Forum => "Forum",
            Self::Academica => "Academica",
            Self::Picknick => "Picknick",
            Self::BonaVista => "Bona Vista",
            Self::GrillCafe => "Grill | Café",
            Self::ZM2 => "ZM2",
            Self::Basilica => "Basilica",
            Self::Atrium => "Atrium",
        }
        .to_string()
    }
}

async fn scrape_menu(url: &str) -> Result<(Vec<Dish>, Vec<Dish>, Vec<Dish>), reqwest::Error> {
    let response = reqwest::get(url).await?;
    let html_content = response.text().await?;

    let document = scraper::Html::parse_document(&html_content);

    let html_main_dishes_selector = scraper::Selector::parse(
        "table.table-dishes.main-dishes > tbody > tr.odd > td.description > div.row > div.desc",
    )
    .unwrap();
    let html_main_dishes = document.select(&html_main_dishes_selector);
    let main_dishes = html_main_dishes
        .filter_map(|dish| Dish::try_from(dish).ok())
        .collect::<Vec<_>>();

    let html_side_dishes_selector = scraper::Selector::parse(
        "table.table-dishes.side-dishes > tbody > tr.odd > td.description > div.row > div.desc",
    )
    .unwrap();
    let html_side_dishes = document.select(&html_side_dishes_selector);
    let side_dishes = html_side_dishes
        .filter_map(|dish| Dish::try_from(dish).ok())
        .collect::<Vec<_>>();

    let html_desserts_selector = scraper::Selector::parse(
        "table.table-dishes.soups > tbody > tr.odd > td.description > div.row > div.desc",
    )
    .unwrap();
    let html_desserts = document.select(&html_desserts_selector);
    let desserts = html_desserts
        .filter_map(|dish| Dish::try_from(dish).ok())
        .collect::<Vec<_>>();

    Ok((main_dishes, side_dishes, desserts))
}
