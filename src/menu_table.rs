use comfy_table::{Cell, CellAlignment, Row, Table};
use itertools::Itertools;

use crate::{cli_args::PriceLevel, Canteen, DailyMenu, Dish};

pub fn menu_table(menu: &[DailyMenu], price_level: Option<PriceLevel>, show_mensa: bool) -> Table {
    let main_dishes = get_dishes(menu, DailyMenu::get_main_dishes);
    let side_dishes = get_dishes(menu, DailyMenu::get_side_dishes);
    let soups = get_dishes(menu, DailyMenu::get_soups);
    let desserts = get_dishes(menu, DailyMenu::get_desserts);
    let other_dishes = get_dishes(menu, DailyMenu::get_other_dishes);

    let mut col_span = if price_level.is_some() { 3 } else { 5 };
    if show_mensa {
        col_span += 1;
    }
    let mut header = vec!["Gericht"];
    if price_level.is_some() {
        header.push("Preis");
    } else {
        header.extend(vec![
            "Preis Studierende",
            "Preis Bedienstete",
            "Preis Gäste",
        ]);
    };
    if show_mensa {
        header.push("Mensa");
    }
    header.push("Extras");

    let mut table = Table::new();
    table
        .load_preset(comfy_table::presets::UTF8_FULL_CONDENSED)
        .set_header(Row::from(header))
        .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth);
    {
        let mut hauptgerichte_row = Row::new();
        hauptgerichte_row.add_cell(
            Cell::from("Hauptgerichte")
                .set_alignment(CellAlignment::Center)
                .add_attribute(comfy_table::Attribute::Underlined)
                .add_attribute(comfy_table::Attribute::OverLined),
        );
        for _ in 0..col_span - 1 {
            hauptgerichte_row.add_cell(
                Cell::new("")
                    .add_attribute(comfy_table::Attribute::Underlined)
                    .add_attribute(comfy_table::Attribute::OverLined),
            );
        }
        table.add_row(hauptgerichte_row);
    }
    for dish in main_dishes {
        table.add_row(into_row(dish.1, &dish.0, price_level, show_mensa));
    }
    {
        let mut beilagen_row = Row::new();
        beilagen_row.add_cell(
            Cell::from("Beilagen")
                .set_alignment(CellAlignment::Center)
                .add_attribute(comfy_table::Attribute::Underlined)
                .add_attribute(comfy_table::Attribute::OverLined),
        );
        for _ in 0..col_span - 1 {
            beilagen_row.add_cell(
                Cell::new("")
                    .add_attribute(comfy_table::Attribute::Underlined)
                    .add_attribute(comfy_table::Attribute::OverLined),
            );
        }
        table.add_row(beilagen_row);
    }
    for dish in side_dishes {
        table.add_row(into_row(dish.1, &dish.0, price_level, show_mensa));
    }
    {
        let mut soups_row = Row::new();
        soups_row.add_cell(
            Cell::from("Suppen")
                .set_alignment(CellAlignment::Center)
                .add_attribute(comfy_table::Attribute::Underlined)
                .add_attribute(comfy_table::Attribute::OverLined),
        );
        for _ in 0..col_span - 1 {
            soups_row.add_cell(
                Cell::new("")
                    .add_attribute(comfy_table::Attribute::Underlined)
                    .add_attribute(comfy_table::Attribute::OverLined),
            );
        }
        table.add_row(soups_row);
    }
    for dish in soups {
        table.add_row(into_row(dish.1, &dish.0, price_level, show_mensa));
    }
    {
        let mut desserts_row = Row::new();
        desserts_row.add_cell(
            Cell::from("Desserts")
                .set_alignment(CellAlignment::Center)
                .add_attribute(comfy_table::Attribute::Underlined)
                .add_attribute(comfy_table::Attribute::OverLined),
        );
        for _ in 0..col_span - 1 {
            desserts_row.add_cell(
                Cell::new("")
                    .add_attribute(comfy_table::Attribute::Underlined)
                    .add_attribute(comfy_table::Attribute::OverLined),
            );
        }
        table.add_row(desserts_row);
    }
    for dish in desserts {
        table.add_row(into_row(dish.1, &dish.0, price_level, show_mensa));
    }
    {
        let mut other_dishes_row = Row::new();
        other_dishes_row.add_cell(
            Cell::from("Andere Gerichte")
                .set_alignment(CellAlignment::Center)
                .add_attribute(comfy_table::Attribute::Underlined)
                .add_attribute(comfy_table::Attribute::OverLined),
        );
        for _ in 0..col_span - 1 {
            other_dishes_row.add_cell(
                Cell::new("")
                    .add_attribute(comfy_table::Attribute::Underlined)
                    .add_attribute(comfy_table::Attribute::OverLined),
            );
        }
        table.add_row(other_dishes_row);
    }
    for dish in other_dishes {
        table.add_row(into_row(dish.1, &dish.0, price_level, show_mensa));
    }
    table
}

fn into_row(
    dish: &Dish,
    mensa: &[&Canteen],
    price_level: Option<PriceLevel>,
    show_mensa: bool,
) -> Row {
    let mut row = Row::new();
    row.add_cell(Cell::from(dish.get_name()).set_alignment(CellAlignment::Left));

    if let Some(price_level) = price_level {
        let price = match price_level {
            PriceLevel::Student => dish.get_price_students(),
            PriceLevel::Bediensteter => dish.get_price_employees(),
            PriceLevel::Gast => dish.get_price_guests(),
        }
        .to_string();
        row.add_cell(Cell::from(price).set_alignment(CellAlignment::Right));
    } else {
        row.add_cell(Cell::from(dish.get_price_students()).set_alignment(CellAlignment::Right))
            .add_cell(Cell::from(dish.get_price_employees()).set_alignment(CellAlignment::Right))
            .add_cell(Cell::from(dish.get_price_guests()).set_alignment(CellAlignment::Right));
    }
    if show_mensa {
        row.add_cell(
            Cell::from(mensa.iter().map(|m| m.to_string()).join(", "))
                .set_alignment(CellAlignment::Right),
        );
    }
    row.add_cell(
        Cell::from(if dish.is_vegan() {
            "vegan"
        } else if dish.is_vegetarian() {
            "vegetarian"
        } else {
            ""
        })
        .set_alignment(CellAlignment::Right),
    );

    row
}

pub fn get_dishes<F>(menu: &[DailyMenu], get: F) -> Vec<(Vec<&Canteen>, &Dish)>
where
    F: Fn(&DailyMenu) -> &[Dish],
{
    menu.iter()
        .flat_map(|m| {
            let mensa = m.get_canteen();
            get(m).iter().map(move |d| (mensa, d)).collect::<Vec<_>>()
        })
        .sorted_by_key(|(_, dish)| dish.get_name())
        .chunk_by(|(_, dish)| *dish)
        .into_iter()
        .map(|(dish, g)| {
            (
                g.into_iter().map(|(mensa, _)| mensa).collect::<Vec<_>>(),
                dish,
            )
        })
        // .filter(|(_, dish)| {
        //     extras.is_empty()
        //         || extras
        //             .iter()
        //             .all(|extra| dish.get_extras().iter().any(|e| e.contains(extra)))
        // })
        .collect::<Vec<_>>()
}
