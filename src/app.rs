use std::{
    collections::HashMap,
    fmt::Display,
    io,
    time::{Duration, Instant},
    vec,
};

use chrono::{Datelike, Duration as CDuration, Utc, Weekday};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use image::DynamicImage;
use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Padding, Paragraph, Row, Table, TableState, Tabs, Wrap},
};
use ratatui_image::StatefulImage;
use strum::{EnumIter, FromRepr, IntoEnumIterator as _};
use tokio::sync::mpsc;

use crate::{menu::Menu, tui, Dish, Mensa};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    selected_tab: SelectedTab,
    selected_item: usize,
    menus: HashMap<SelectedTab, Menu>,
    imgs: HashMap<String, DynamicImage>,
}

#[derive(Default, Clone, Copy, Debug, FromRepr, EnumIter, PartialEq, Eq, Hash)]
enum SelectedTab {
    #[default]
    Today,
    Tomorrow,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
}

impl Display for SelectedTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(get_day_display(*self as i64))
    }
}

impl App {
    /// runs the application's main loop until the user quits
    pub async fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        let (tx_menu, mut rx_menu) = mpsc::channel(10);
        let (tx_img, mut rx_img) = mpsc::channel(50);

        for tab in SelectedTab::iter() {
            let tx_menu = tx_menu.clone();
            let tx_img = tx_img.clone();

            tokio::spawn(async move {
                let menu = Menu::new(
                    (Utc::now() + CDuration::days(tab as i64)).date_naive(),
                    &[Mensa::Academica, Mensa::Forum],
                    tx_img,
                )
                .await
                .unwrap();
                tx_menu.send((tab, menu)).await.unwrap();
            });
        }

        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            self.handle_events(timeout)?;

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }

            while let Ok((tab, menu)) = rx_menu.try_recv() {
                self.menus.insert(tab, menu);
            }
            while let Ok((name, img)) = rx_img.try_recv() {
                self.imgs.insert(name, img);
            }
        }
        Ok(())
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
        self.selected_item = 0;
    }

    pub fn prev_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
        self.selected_item = 0;
    }

    pub fn next_item(&mut self) {
        let amount = self
            .menus
            .get(&self.selected_tab)
            .map(|m| m.get_main_dishes().len() + m.get_side_dishes().len() + m.get_desserts().len())
            .unwrap_or_default();
        if self.selected_item + 1 < amount {
            self.selected_item += 1;
        }
    }

    pub fn prev_item(&mut self) {
        self.selected_item = self.selected_item.saturating_sub(1);
    }

    fn handle_events(&mut self, timeout: Duration) -> io::Result<()> {
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left | KeyCode::Char('h') => self.prev_tab(),
            KeyCode::Right | KeyCode::Char('l') => self.next_tab(),
            KeyCode::Up | KeyCode::Char('k') => self.prev_item(),
            KeyCode::Down | KeyCode::Char('j') => self.next_item(),
            _ => {}
        }
    }
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

impl App {
    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), Color::Green);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }

    fn render_tab(&self, tab: SelectedTab, area: Rect, buf: &mut Buffer) {
        let outer_block = Block::bordered()
            .border_set(symbols::border::ROUNDED)
            .padding(Padding::horizontal(1));

        let [details, overview] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(35), Constraint::Fill(1)])
            .areas(outer_block.inner(area));

        outer_block.render(area, buf);

        let details_block = Block::default()
            .title("Details")
            .borders(Borders::ALL)
            .padding(Padding::proportional(1));
        let details_inner = details_block.inner(details);
        details_block.render(details, buf);

        if let Some(dish) = self.menus.get(&self.selected_tab).and_then(|m| {
            m.get_main_dishes()
                .iter()
                .chain(m.get_side_dishes())
                .chain(m.get_desserts())
                .nth(self.selected_item)
        }) {
            self.render_dish(dish, details_inner, buf);
        } else {
            Paragraph::new("Kein Gericht ausgewählt").render(details_inner, buf);
        }

        let overview_block = Block::default()
            .title("Overview")
            .padding(Padding::proportional(1))
            .borders(Borders::ALL);
        let overview_inner = overview_block.inner(overview);
        overview_block.render(overview, buf);

        if let Some(menu) = self.menus.get(&tab) {
            self.render_menu(menu, overview_inner, buf);
        } else {
            Paragraph::new("Loading...").render(overview_inner, buf);
        }
    }

    fn render_menu(&self, menu: &Menu, area: Rect, buf: &mut Buffer) {
        let main_dishes = menu.get_main_dishes();
        let side_dishes = menu.get_side_dishes();
        let desserts = menu.get_desserts();

        if main_dishes.is_empty() {
            Paragraph::new("Die Mensa hat geschlossen").render(area, buf);
        } else {
            let mut state = TableState::default().with_selected(self.selected_item);
            StatefulWidget::render(
                Table::new(
                    main_dishes
                        .iter()
                        .chain(side_dishes)
                        .chain(desserts)
                        .map(Row::from),
                    vec![Constraint::Fill(1), Constraint::Length(6)],
                )
                .header(Row::new(vec!["Gericht", "Mensen"]))
                .highlight_style(Style::new().blue().on_dark_gray())
                .highlight_symbol("> "),
                area,
                buf,
                &mut state,
            );
        }
    }

    fn render_dish(&self, dish: &Dish, area: Rect, buf: &mut Buffer) {
        let [name_area, image_area, prices_area, mensen_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .areas(area);

        Paragraph::new(dish.get_name())
            .centered()
            .wrap(Wrap::default())
            .render(name_area, buf);

        if let Some(img) = self.imgs.get(dish.get_name()) {
            let mut picker = ratatui_image::picker::Picker::new((8, 12));
            picker.guess_protocol();

            let mut img = picker.new_resize_protocol(img.clone());
            let widget = StatefulImage::new(None);
            StatefulWidget::render(widget, image_area, buf, &mut img);
        }

        let prices_table = Table::new(
            vec![
                Row::new(vec!["Studenten:", dish.get_price_students().unwrap_or("-")]),
                Row::new(vec![
                    "Mitarbeiter:",
                    dish.get_price_employees().unwrap_or("-"),
                ]),
                Row::new(vec!["Gäste:", dish.get_price_guests().unwrap_or("-")]),
            ],
            vec![Constraint::Min(12), Constraint::Min(6)],
        )
        .header(Row::new(vec!["", "Preis:"]));

        Widget::render(prices_table, prices_area, buf);

        Layout::vertical(dish.get_mensen().iter().map(|_| Constraint::Length(1)))
            .split(mensen_area)
            .iter()
            .zip(dish.get_mensen().iter().map(Mensa::to_string).sorted())
            .for_each(|(area, mensa)| {
                Paragraph::new(mensa).render(*area, buf);
            });
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);
        let horizontal = Layout::horizontal([Constraint::Min(0), Constraint::Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        self.render_tab(self.selected_tab, inner_area, buf);
        render_footer(footer_area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "Mensa UPB".bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("▲▼ to change item | ◄ ► to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

impl SelectedTab {
    /// Return tab's name as a styled `Line`
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(Color::Gray)
            .bg(Color::DarkGray)
            .into()
    }
}

fn get_day_display(offset: i64) -> &'static str {
    match offset {
        0 => "Heute",
        1 => "Morgen",
        _ => {
            let future_date = Utc::now() + CDuration::days(offset);
            let weekday = future_date.weekday();

            match weekday {
                Weekday::Mon => "Montag",
                Weekday::Tue => "Dienstag",
                Weekday::Wed => "Mittwoch",
                Weekday::Thu => "Donnerstag",
                Weekday::Fri => "Freitag",
                Weekday::Sat => "Samstag",
                Weekday::Sun => "Sonntag",
            }
        }
    }
}

impl<'a> From<&'a Dish> for Row<'a> {
    fn from(value: &'a Dish) -> Self {
        let mensen_display = value
            .get_mensen()
            .iter()
            .sorted()
            .map(Mensa::get_char)
            .sorted()
            .join("");
        Row::new(vec![value.get_name().to_string(), mensen_display])
    }
}
