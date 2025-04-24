use crate::{menu_table::get_dishes, DailyMenu};

pub fn generate_json(menu: &[DailyMenu], extras: Vec<String>) -> String {
    let main_dishes = get_dishes(menu, DailyMenu::get_main_dishes, &extras);
    let side_dishes = get_dishes(menu, DailyMenu::get_side_dishes, &extras);
    let desserts = get_dishes(menu, DailyMenu::get_desserts, &extras);

    #[derive(Eq, PartialEq, Copy, Clone, Hash, serde::Serialize)]
    enum DishType {
        Main,
        Side,
        Dessert,
    }

    let mut output = std::collections::HashMap::<DishType, Vec<_>>::new();

    for (_, dish) in main_dishes {
        output.entry(DishType::Main).or_default().push(dish);
    }

    for (_, dish) in side_dishes {
        output.entry(DishType::Side).or_default().push(dish);
    }

    for (_, dish) in desserts {
        output.entry(DishType::Dessert).or_default().push(dish);
    }

    serde_json::to_string(&output).unwrap()
}
