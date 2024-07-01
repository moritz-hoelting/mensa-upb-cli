use std::io::Cursor;

use image::DynamicImage;
use itertools::Itertools;
use scraper::ElementRef;
use tokio::sync::mpsc;

use crate::Mensa;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dish {
    name: String,
    price_students: Option<String>,
    price_employees: Option<String>,
    price_guests: Option<String>,
    extras: Vec<String>,
    mensen: Vec<Mensa>,
}

impl Dish {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_price_students(&self) -> Option<&str> {
        self.price_students.as_deref()
    }
    pub fn get_price_employees(&self) -> Option<&str> {
        self.price_employees.as_deref()
    }
    pub fn get_price_guests(&self) -> Option<&str> {
        self.price_guests.as_deref()
    }
    pub fn get_extras(&self) -> &[String] {
        &self.extras
    }
    pub fn get_mensen(&self) -> &[Mensa] {
        &self.mensen
    }

    pub fn same_as(&self, other: &Self) -> bool {
        self.name == other.name
            && self.price_employees == other.price_employees
            && self.price_guests == other.price_guests
            && self.price_students == other.price_students
            && self.extras.iter().sorted().collect_vec()
                == self.extras.iter().sorted().collect_vec()
    }

    pub fn merge(&mut self, other: Self) {
        self.mensen.extend(other.mensen);
        self.mensen.sort();
        self.mensen.dedup();
    }
}

impl Dish {
    pub fn from_element(
        element: ElementRef,
        tx: mpsc::Sender<(String, DynamicImage)>,
        mensa: Mensa,
    ) -> Option<Self> {
        let html_name_selector = scraper::Selector::parse(".desc h4").unwrap();
        let name = element
            .select(&html_name_selector)
            .next()?
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();

        let img_selector = scraper::Selector::parse(".img img").unwrap();
        let img_src_path = element.select(&img_selector).next()?.value().attr("src")?;
        let img_src = format!("https://www.studierendenwerk-pb.de/{}", img_src_path);

        let name_clone = name.clone();
        tokio::spawn(async move {
            if let Ok(img) = reqwest::get(img_src).await {
                if let Ok(img_bytes) = img.bytes().await {
                    if let Some(dyn_img) = image::io::Reader::new(Cursor::new(img_bytes))
                        .with_guessed_format()
                        .ok()
                        .and_then(|r| r.decode().ok())
                    {
                        let _ = tx.send((name_clone, dyn_img)).await;
                    }
                }
            }
        });

        let html_price_selector = scraper::Selector::parse(".desc .price").unwrap();
        let mut prices = element
            .select(&html_price_selector)
            .filter_map(|price| {
                let price_for = price.first_child().and_then(|strong| {
                    strong.first_child().and_then(|text_element| {
                        text_element
                            .value()
                            .as_text()
                            .map(|text| text.trim().trim_end_matches(':').to_string())
                    })
                });
                let price_value = price.last_child().and_then(|text_element| {
                    text_element
                        .value()
                        .as_text()
                        .map(|text| text.trim().to_string())
                });
                price_for
                    .and_then(|price_for| price_value.map(|price_value| (price_for, price_value)))
            })
            .collect::<Vec<_>>();

        let html_extras_selector = scraper::Selector::parse(".desc .buttons > *").unwrap();
        let extras = element
            .select(&html_extras_selector)
            .filter_map(|extra| extra.value().attr("title").map(|title| title.to_string()))
            .collect::<Vec<_>>();

        Some(Self {
            name,
            price_students: prices
                .iter_mut()
                .find(|(price_for, _)| price_for == "Studierende")
                .map(|(_, price)| std::mem::take(price)),
            price_employees: prices
                .iter_mut()
                .find(|(price_for, _)| price_for == "Bedienstete")
                .map(|(_, price)| std::mem::take(price)),
            price_guests: prices
                .iter_mut()
                .find(|(price_for, _)| price_for == "Gäste")
                .map(|(_, price)| std::mem::take(price)),
            extras,
            mensen: vec![mensa],
        })
    }
}

impl PartialOrd for Dish {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}
