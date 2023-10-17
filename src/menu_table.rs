use comfy_table::{Cell, CellAlignment, Row, Table};
use itertools::Itertools;

use crate::{cli_args::PriceLevel, DailyMenu, Dish, Mensa};

pub fn menu_table(
    menu: &[DailyMenu],
    price_level: Option<PriceLevel>,
    show_mensa: bool,
    extras: Vec<String>,
) -> Table {
    let main_dishes = get_dishes(menu, DailyMenu::get_main_dishes, &extras);
    let side_dishes = get_dishes(menu, DailyMenu::get_side_dishes, &extras);
    let desserts = get_dishes(menu, DailyMenu::get_desserts, &extras);

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

    table
}

fn into_row(
    dish: &Dish,
    mensa: &[&Mensa],
    price_level: Option<PriceLevel>,
    show_mensa: bool,
) -> Row {
    let mut row = Row::new();
    row.add_cell(Cell::from(dish.get_name()).set_alignment(CellAlignment::Left));

    if let Some(price_level) = price_level {
        let price = match price_level {
            PriceLevel::Student => dish.get_price_students().unwrap_or("-"),
            PriceLevel::Bediensteter => dish.get_price_employees().unwrap_or("-"),
            PriceLevel::Gast => dish.get_price_guests().unwrap_or("-"),
        }
        .to_string();
        row.add_cell(Cell::from(price).set_alignment(CellAlignment::Right));
    } else {
        row.add_cell(
            Cell::from(dish.get_price_students().unwrap_or_default())
                .set_alignment(CellAlignment::Right),
        )
        .add_cell(
            Cell::from(dish.get_price_employees().unwrap_or_default())
                .set_alignment(CellAlignment::Right),
        )
        .add_cell(
            Cell::from(dish.get_price_guests().unwrap_or_default())
                .set_alignment(CellAlignment::Right),
        );
    }
    if show_mensa {
        row.add_cell(
            Cell::from(mensa.iter().map(|m| m.to_string()).join(", "))
                .set_alignment(CellAlignment::Right),
        );
    }
    row.add_cell(Cell::from(dish.get_extras().join(", ")).set_alignment(CellAlignment::Right));

    row
}

fn get_dishes<'a, F>(
    menu: &'a [DailyMenu],
    get: F,
    extras: &[String],
) -> Vec<(Vec<&'a Mensa>, &'a Dish)>
where
    F: Fn(&DailyMenu) -> &[Dish],
{
    menu.iter()
        .flat_map(|m| {
            let mensa = m.get_mensa();
            get(m).iter().map(move |d| (mensa, d)).collect::<Vec<_>>()
        })
        .sorted_by_key(|(_, dish)| dish.get_name())
        .group_by(|(_, dish)| *dish)
        .into_iter()
        .map(|(dish, g)| {
            (
                g.into_iter().map(|(mensa, _)| mensa).collect::<Vec<_>>(),
                dish,
            )
        })
        .filter(|(_, dish)| {
            extras.is_empty()
                || extras
                    .iter()
                    .all(|extra| dish.get_extras().iter().any(|e| e.contains(extra)))
        })
        .collect::<Vec<_>>()
}
