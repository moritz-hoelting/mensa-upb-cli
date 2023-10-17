use scraper::ElementRef;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dish {
    name: String,
    price_students: Option<String>,
    price_employees: Option<String>,
    price_guests: Option<String>,
    extras: Vec<String>,
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
}

impl TryFrom<ElementRef<'_>> for Dish {
    type Error = ();

    fn try_from(value: ElementRef) -> Result<Self, Self::Error> {
        let html_name_selector = scraper::Selector::parse("h4").unwrap();
        let name = value
            .select(&html_name_selector)
            .next()
            .ok_or(())?
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();

        let html_price_selector = scraper::Selector::parse(".price").unwrap();
        let mut prices = value
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

        let html_extras_selector = scraper::Selector::parse(".buttons > *").unwrap();
        let extras = value
            .select(&html_extras_selector)
            .filter_map(|extra| extra.value().attr("title").map(|title| title.to_string()))
            .collect::<Vec<_>>();

        Ok(Self {
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
        })
    }
}

impl PartialOrd for Dish {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}
