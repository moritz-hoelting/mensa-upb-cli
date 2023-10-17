use itertools::Itertools;
use term_data_table::{Alignment, Cell, Row, Table, TableStyle};

use crate::{cli_args::PriceLevel, DailyMenu, Dish, Mensa};

pub fn menu_table(menu: &[DailyMenu], price_level: Option<PriceLevel>, show_mensa: bool) -> Table {
    let main_dishes = get_dishes(menu, DailyMenu::get_main_dishes);
    let side_dishes = get_dishes(menu, DailyMenu::get_side_dishes);
    let desserts = get_dishes(menu, DailyMenu::get_desserts);

    let mut col_span = if price_level.is_some() { 3 } else { 5 };
    if show_mensa {
        col_span += 1;
    }
    let mut header = Row::new().with_cell(Cell::from("Gericht"));
    if price_level.is_some() {
        header.add_cell(Cell::from("Preis"));
    } else {
        header
            .add_cell(Cell::from("Preis Studierende"))
            .add_cell(Cell::from("Preis Bedienstete"))
            .add_cell(Cell::from("Preis Gäste"));
    };
    if show_mensa {
        header.add_cell(Cell::from("Mensa"));
    }
    header.add_cell(Cell::from("Extras"));

    let mut table = Table::new()
        .with_style(TableStyle::THIN)
        .with_row(header)
        .with_row(
            Row::new().with_cell(
                Cell::from("Hauptgerichte")
                    .with_alignment(Alignment::Center)
                    .with_col_span(col_span),
            ),
        );
    for dish in main_dishes {
        table.add_row(into_filtered_price_row(
            dish.1,
            &dish.0,
            price_level,
            show_mensa,
        ));
    }
    table.add_row(
        Row::new().with_cell(
            Cell::from("Beilagen")
                .with_alignment(Alignment::Center)
                .with_col_span(col_span),
        ),
    );
    for dish in side_dishes {
        table.add_row(into_filtered_price_row(
            dish.1,
            &dish.0,
            price_level,
            show_mensa,
        ));
    }
    table.add_row(
        Row::new().with_cell(
            Cell::from("Desserts")
                .with_alignment(Alignment::Center)
                .with_col_span(col_span),
        ),
    );
    for dish in desserts {
        table.add_row(into_filtered_price_row(
            dish.1,
            &dish.0,
            price_level,
            show_mensa,
        ));
    }

    table
}

fn into_filtered_price_row<'a>(
    dish: &'a Dish,
    mensa: &[&'a Mensa],
    price_level: Option<PriceLevel>,
    show_mensa: bool,
) -> Row<'a> {
    let mut row = Row::new().with_cell(Cell::from(dish.get_name()).with_alignment(Alignment::Left));

    if let Some(price_level) = price_level {
        let price = match price_level {
            PriceLevel::Student => dish.get_price_students().unwrap_or("-"),
            PriceLevel::Bediensteter => dish.get_price_employees().unwrap_or("-"),
            PriceLevel::Gast => dish.get_price_guests().unwrap_or("-"),
        }
        .to_string();
        row.add_cell(Cell::from(price).with_alignment(Alignment::Right));
    } else {
        row.add_cell(
            Cell::from(dish.get_price_students().unwrap_or_default())
                .with_alignment(Alignment::Right),
        )
        .add_cell(
            Cell::from(dish.get_price_employees().unwrap_or_default())
                .with_alignment(Alignment::Right),
        )
        .add_cell(
            Cell::from(dish.get_price_guests().unwrap_or_default())
                .with_alignment(Alignment::Right),
        );
    }
    if show_mensa {
        row.add_cell(
            Cell::from(mensa.iter().map(|m| m.to_string()).join(", "))
                .with_alignment(Alignment::Left),
        );
    }
    row.add_cell(Cell::from(dish.get_extras().join(", ")).with_alignment(Alignment::Left));

    row
}

fn get_dishes<F>(menu: &[DailyMenu], get: F) -> Vec<(Vec<&Mensa>, &Dish)>
where
    F: Fn(&DailyMenu) -> &[Dish],
{
    menu.iter()
        .flat_map(|m| {
            let mensa = m.get_mensa();
            get(m).iter().map(move |d| (mensa, d)).collect::<Vec<_>>()
        })
        .sorted_by_key(|d| d.1.get_name())
        .group_by(|d| d.1)
        .into_iter()
        .map(|(dish, g)| {
            (
                g.into_iter().map(|(mensa, _)| mensa).collect::<Vec<_>>(),
                dish,
            )
        })
        .collect::<Vec<_>>()
}
