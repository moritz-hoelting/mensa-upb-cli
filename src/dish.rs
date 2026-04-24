use rust_decimal::Decimal;
use std::{borrow::Cow, fmt::Display};

use crate::{
    daily_menu::ResponseMeal,
    util::{first_non_empty_string, normalize_price_bigdecimal},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct Dish {
    pub name: String,
    pub image_src: Option<String>,
    pub price_students: Decimal,
    pub price_employees: Decimal,
    pub price_guests: Decimal,
    pub vegetarian: bool,
    pub vegan: bool,
    pub dish_type: DishType,
    pub nutrition_values: NutritionValues,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, serde::Serialize)]
pub struct NutritionValues {
    pub kjoule: Option<i32>,
    pub protein: Option<Decimal>,
    pub carbs: Option<Decimal>,
    pub fat: Option<Decimal>,
    pub saturated_fat: Option<Decimal>,
}

impl Dish {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_price_students(&self) -> &Decimal {
        &self.price_students
    }
    pub fn get_price_employees(&self) -> &Decimal {
        &self.price_employees
    }
    pub fn get_price_guests(&self) -> &Decimal {
        &self.price_guests
    }
    pub fn get_image_src(&self) -> Option<&str> {
        self.image_src.as_deref()
    }
    pub fn is_vegan(&self) -> bool {
        self.vegan
    }
    pub fn is_vegetarian(&self) -> bool {
        self.vegetarian
    }
    pub fn get_type(&self) -> DishType {
        self.dish_type
    }

    pub fn same_as(&self, other: &Self) -> bool {
        self.name == other.name
            && self.price_employees == other.price_employees
            && self.price_guests == other.price_guests
            && self.price_students == other.price_students
            && self.vegan == other.vegan
            && self.vegetarian == other.vegetarian
            && self.dish_type == other.dish_type
    }
}

impl NutritionValues {
    pub fn normalize(self) -> Self {
        Self {
            kjoule: self.kjoule,
            protein: self.protein.map(|p| p.normalize().round_dp(2)),
            carbs: self.carbs.map(|c| c.normalize().round_dp(2)),
            fat: self.fat.map(|f| f.normalize().round_dp(2)),
            saturated_fat: self.saturated_fat.map(|sf| sf.normalize().round_dp(2)),
        }
    }
}

impl PartialOrd for Dish {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl From<ResponseMeal> for Dish {
    fn from(meal: ResponseMeal) -> Self {
        let vegan = meal.is_vegan();
        let vegetarian = meal.is_vegetarian();
        Self {
            name: match html_escape::decode_html_entities(&meal.title) {
                Cow::Owned(o) => o,
                Cow::Borrowed(_) => meal.title,
            },
            image_src: first_non_empty_string([
                meal.image_jpeg,
                meal.image_jpeg_small,
                meal.image_jpeg_thumb,
                meal.image_webp,
                meal.image_webp_small,
                meal.image_webp_thumb,
            ]),
            price_students: price_to_bigdecimal(&meal.price_students),
            price_employees: price_to_bigdecimal(&meal.price_staff),
            price_guests: price_to_bigdecimal(&meal.price_guests),
            vegan,
            vegetarian,
            dish_type: DishType::from_category(meal.category.as_str()),
            nutrition_values: nutrition_from_str(&meal.nutrition),
        }
    }
}

fn price_to_bigdecimal(s: &str) -> Decimal {
    s.replace(',', ".")
        .parse::<Decimal>()
        .ok()
        .map(normalize_price_bigdecimal)
        .unwrap_or_else(|| Decimal::from(99999))
}

impl ResponseMeal {
    fn is_vegan(&self) -> bool {
        self.button.to_lowercase().contains("/4.png")
    }

    fn is_vegetarian(&self) -> bool {
        self.button.to_lowercase().contains("/3.png") || self.is_vegan()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize)]
pub enum DishType {
    Main,
    Side,
    Soup,
    Dessert,
    Other,
}

impl Display for DishType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Main => "main",
            Self::Side => "side",
            Self::Soup => "soup",
            Self::Dessert => "dessert",
            Self::Other => "other",
        };
        f.write_str(s)
    }
}

impl DishType {
    fn from_category(category: &str) -> Self {
        if category.trim().is_empty() {
            return Self::Other;
        }
        let lower = category.to_lowercase();

        MEAL_CATEGORY_PATTERNS
            .iter()
            .find(|pattern| (pattern.test)(&lower))
            .map(|pattern| pattern.dish_type)
            .unwrap_or(DishType::Other)
    }
}

const MEAL_CATEGORY_PATTERNS: [MealCategoryPattern; 4] = [
    MealCategoryPattern {
        test: |s| s.contains("eintopf") || s.contains("suppe"),
        dish_type: DishType::Soup,
    },
    MealCategoryPattern {
        test: |s| s.contains("beilage") || s.contains("sättigungbeil") || s.contains("gemüsebeil"),
        dish_type: DishType::Side,
    },
    MealCategoryPattern {
        test: |s| s.contains("dessert"),
        dish_type: DishType::Dessert,
    },
    MealCategoryPattern {
        test: |s| {
            s.contains("fleisch")
                || s.contains("fisch")
                || s.contains("vegetarisch")
                || s.contains("vegan")
                || s.contains("aktions")
                || s.contains("pasta")
                || s.contains("cafeteria")
                || s.contains("zwischenverpflegung")
                || s.contains("restanten")
                || s.contains("bona vista")
        },
        dish_type: DishType::Main,
    },
];

struct MealCategoryPattern {
    dish_type: DishType,
    test: fn(&str) -> bool,
}

fn nutrition_from_str(nutrition_str: &str) -> NutritionValues {
    if !nutrition_str.trim().is_empty() {
        let regex_kjoule = lazy_regex::regex!(r"Brennwert=(\d+) kJ"i);
        let regex_protein = lazy_regex::regex!(r"Eiweiß=(\d+(?:,\d+)?)g"i);
        let regex_carbs = lazy_regex::regex!(r"Kohlenhydrate=(\d+(?:,\d+)?)g"i);
        let regex_fat = lazy_regex::regex!(r"Fett=(\d+(?:,\d+)?)g"i);
        let regex_saturated_fat =
            lazy_regex::regex!(r"davon gesättigte Fettsäuren=(\d+(?:,\d+)?)g"i);

        let kjoule = regex_kjoule
            .captures(nutrition_str)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse().ok());

        let protein = regex_protein
            .captures(nutrition_str)
            .and_then(|c| c.get(1))
            .and_then(|m| grams_to_bigdecimal(m.as_str()));

        let carbs = regex_carbs
            .captures(nutrition_str)
            .and_then(|c| c.get(1))
            .and_then(|m| grams_to_bigdecimal(m.as_str()));

        let fat = regex_fat
            .captures(nutrition_str)
            .and_then(|c| c.get(1))
            .and_then(|m| grams_to_bigdecimal(m.as_str()));

        let saturated_fat = regex_saturated_fat
            .captures(nutrition_str)
            .and_then(|c| c.get(1))
            .and_then(|m| grams_to_bigdecimal(m.as_str()));

        NutritionValues {
            kjoule,
            protein,
            carbs,
            fat,
            saturated_fat,
        }
    } else {
        NutritionValues::default()
    }
}

fn grams_to_bigdecimal(s: &str) -> Option<Decimal> {
    s.trim_end_matches("g")
        .replace(',', ".")
        .trim()
        .parse()
        .ok()
}
