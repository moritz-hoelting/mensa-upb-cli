use crate::{dish::DishType, menu_table::get_dishes, DailyMenu};

pub fn generate_json(menu: &[DailyMenu]) -> String {
    let main_dishes = get_dishes(menu, DailyMenu::get_main_dishes);
    let side_dishes = get_dishes(menu, DailyMenu::get_side_dishes);
    let soups = get_dishes(menu, DailyMenu::get_soups);
    let desserts = get_dishes(menu, DailyMenu::get_desserts);
    let other_dishes = get_dishes(menu, DailyMenu::get_other_dishes);

    let mut output = std::collections::HashMap::<DishType, Vec<_>>::new();

    for (_, dish) in main_dishes {
        output.entry(DishType::Main).or_default().push(dish);
    }

    for (_, dish) in side_dishes {
        output.entry(DishType::Side).or_default().push(dish);
    }

    for (_, dish) in soups {
        output.entry(DishType::Soup).or_default().push(dish);
    }

    for (_, dish) in desserts {
        output.entry(DishType::Dessert).or_default().push(dish);
    }

    for (_, dish) in other_dishes {
        output.entry(DishType::Other).or_default().push(dish);
    }

    serde_json::to_string(&output).unwrap()
}
