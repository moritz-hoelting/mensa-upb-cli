use chrono::NaiveDate;
use image::DynamicImage;
use tokio::sync::mpsc;

use crate::{Dish, Mensa};

#[derive(Debug, Clone)]
pub struct Menu {
    main_dishes: Vec<Dish>,
    side_dishes: Vec<Dish>,
    desserts: Vec<Dish>,
}

impl Menu {
    pub async fn new(
        day: NaiveDate,
        mensen: &[Mensa],
        tx: mpsc::Sender<(String, DynamicImage)>,
    ) -> Result<Self, reqwest::Error> {
        let mut main_dishes = Vec::new();
        let mut side_dishes = Vec::new();
        let mut desserts = Vec::new();

        for mensa in mensen.iter().copied() {
            let (main, side, des) = scrape_menu(mensa, day, tx.clone()).await?;
            for dish in main {
                if let Some(existing) = main_dishes.iter_mut().find(|d| dish.same_as(d)) {
                    existing.merge(dish);
                } else {
                    main_dishes.push(dish);
                }
            }
            for dish in side {
                if let Some(existing) = side_dishes.iter_mut().find(|d| dish.same_as(d)) {
                    existing.merge(dish);
                } else {
                    side_dishes.push(dish);
                }
            }
            for dish in des {
                if let Some(existing) = desserts.iter_mut().find(|d| dish.same_as(d)) {
                    existing.merge(dish);
                } else {
                    desserts.push(dish);
                }
            }
        }

        let compare_name = |a: &Dish, b: &Dish| a.get_name().cmp(b.get_name());

        main_dishes.sort_by(compare_name);
        side_dishes.sort_by(compare_name);
        desserts.sort_by(compare_name);

        Ok(Self {
            main_dishes,
            side_dishes,
            desserts,
        })
    }

    pub fn get_main_dishes(&self) -> &[Dish] {
        &self.main_dishes
    }

    pub fn get_side_dishes(&self) -> &[Dish] {
        &self.side_dishes
    }

    pub fn get_desserts(&self) -> &[Dish] {
        &self.desserts
    }
}

async fn scrape_menu(
    mensa: Mensa,
    day: NaiveDate,
    tx: mpsc::Sender<(String, DynamicImage)>,
) -> Result<(Vec<Dish>, Vec<Dish>, Vec<Dish>), reqwest::Error> {
    let url = mensa.get_url();
    let client = reqwest::Client::new();
    let request_builder = client
        .post(url)
        .query(&[("tx_pamensa_mensa[date]", day.format("%Y-%m-%d").to_string())]);
    let response = request_builder.send().await?;
    let html_content = response.text().await?;

    let document = scraper::Html::parse_document(&html_content);

    let html_main_dishes_selector = scraper::Selector::parse(
        "table.table-dishes.main-dishes > tbody > tr.odd > td.description > div.row",
    )
    .unwrap();
    let html_main_dishes = document.select(&html_main_dishes_selector);
    let main_dishes = html_main_dishes
        .filter_map(|dish| Dish::from_element(dish, tx.clone(), mensa))
        .collect::<Vec<_>>();

    let html_side_dishes_selector = scraper::Selector::parse(
        "table.table-dishes.side-dishes > tbody > tr.odd > td.description > div.row",
    )
    .unwrap();
    let html_side_dishes = document.select(&html_side_dishes_selector);
    let side_dishes = html_side_dishes
        .filter_map(|dish| Dish::from_element(dish, tx.clone(), mensa))
        .collect::<Vec<_>>();

    let html_desserts_selector = scraper::Selector::parse(
        "table.table-dishes.soups > tbody > tr.odd > td.description > div.row",
    )
    .unwrap();
    let html_desserts = document.select(&html_desserts_selector);
    let desserts = html_desserts
        .filter_map(|dish| Dish::from_element(dish, tx.clone(), mensa))
        .collect::<Vec<_>>();

    Ok((main_dishes, side_dishes, desserts))
}
