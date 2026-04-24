use std::collections::BTreeMap;

use chrono::NaiveDate;
use itertools::Itertools;

use crate::{dish::DishType, Canteen, Dish};

const API_URL: &str = "https://stwpb.de/wp-json/stwk-pb/v1/meals";

#[derive(Debug, Clone, PartialEq)]
pub struct DailyMenu {
    canteen: Canteen,
    main_dishes: Vec<Dish>,
    side_dishes: Vec<Dish>,
    soups: Vec<Dish>,
    desserts: Vec<Dish>,
    other_dishes: Vec<Dish>,
}

impl DailyMenu {
    pub async fn scrape(
        start_date: &NaiveDate,
        end_date: &NaiveDate,
        canteen: Canteen,
    ) -> Result<Self, reqwest::Error> {
        let scraped = scrape_menu(start_date, end_date, canteen).await?;

        let chunked = scraped
            .into_iter()
            .sorted_by_key(|x| x.dish_type)
            .chunk_by(|x| x.dish_type);

        let mut dishes = chunked
            .into_iter()
            .map(|c| (c.0, c.1.collect()))
            .collect::<BTreeMap<_, Vec<_>>>();

        let main_dishes =
            std::mem::take(dishes.get_mut(&DishType::Main).unwrap_or(&mut Vec::new()));
        let side_dishes =
            std::mem::take(dishes.get_mut(&DishType::Side).unwrap_or(&mut Vec::new()));
        let soups = std::mem::take(dishes.get_mut(&DishType::Soup).unwrap_or(&mut Vec::new()));
        let desserts = std::mem::take(
            dishes
                .get_mut(&DishType::Dessert)
                .unwrap_or(&mut Vec::new()),
        );
        let other_dishes =
            std::mem::take(dishes.get_mut(&DishType::Other).unwrap_or(&mut Vec::new()));

        Ok(DailyMenu {
            canteen,
            main_dishes,
            side_dishes,
            desserts,
            soups,
            other_dishes,
        })
    }

    pub fn get_canteen(&self) -> &Canteen {
        &self.canteen
    }
    pub fn get_main_dishes(&self) -> &[Dish] {
        &self.main_dishes
    }
    pub fn get_side_dishes(&self) -> &[Dish] {
        &self.side_dishes
    }
    pub fn get_soups(&self) -> &[Dish] {
        &self.soups
    }
    pub fn get_desserts(&self) -> &[Dish] {
        &self.desserts
    }
    pub fn get_other_dishes(&self) -> &[Dish] {
        &self.other_dishes
    }
}

pub async fn scrape_menu(
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    canteen: Canteen,
) -> Result<Vec<Dish>, reqwest::Error> {
    let client = reqwest::Client::new();
    let request_builder = client.get(API_URL).query(&[
        ("venue", canteen.get_venue_id().to_string()),
        ("start_date", start_date.format("%Y-%m-%d").to_string()),
        ("end_date", end_date.format("%Y-%m-%d").to_string()),
    ]);
    let response = request_builder.send().await?;
    let response_data = response.json::<ResponseData>().await?;

    let res = response_data.meals.into_iter().map(Dish::from).collect();

    Ok(res)
}

#[expect(dead_code)]
#[derive(Debug, Clone, serde::Deserialize)]
struct ResponseData {
    venue: String,
    venue_name: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
    meals: Vec<ResponseMeal>,
    total: usize,
}

#[expect(dead_code)]
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct ResponseMeal {
    pub id: usize,
    pub title: String,
    pub date: NaiveDate,
    pub date_german: String,
    pub category: String,
    pub price_students: String,
    pub price_staff: String,
    pub price_guests: String,
    pub allergens_raw: String,
    pub allergens_decoded: ResponseAllergensDecoded,
    pub nutrition: String,
    pub button: String,
    pub image_jpeg: String,
    pub image_webp: String,
    pub image_jpeg_small: String,
    pub image_webp_small: String,
    pub image_jpeg_thumb: String,
    pub image_webp_thumb: String,
}

#[expect(dead_code)]
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct ResponseAllergensDecoded {
    pub allergens: Vec<ResponseAllergen>,
    pub additives: Vec<ResponseAdditive>,
    pub raw_codes: Vec<String>,
}

#[expect(dead_code)]
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct ResponseAllergen {
    pub id: String,
    pub code: String,
    pub name_de: String,
    pub name_en: String,
    pub category: String,
    pub active: String,
    pub sort_order: String,
}

#[expect(dead_code)]
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct ResponseAdditive {
    pub id: String,
    pub code: String,
    pub name_de: String,
    pub name_en: String,
    pub active: String,
    pub sort_order: String,
}
