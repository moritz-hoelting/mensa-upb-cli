use std::{
    collections::HashMap,
    fmt::Display,
    io,
    sync::Arc,
    time::{Duration, Instant},
};

use chrono::{Datelike, Duration as CDuration, Utc, Weekday};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Padding, Paragraph, Tabs},
};
use strum::{EnumIter, FromRepr, IntoEnumIterator as _};
use tokio::sync::{mpsc, RwLock};

use crate::{menu::Menu, tui, Mensa};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    selected_tab: SelectedTab,
    menus: Arc<RwLock<HashMap<SelectedTab, Menu>>>,
}

#[derive(Debug, Default)]
pub struct AppWidget {
    selected_tab: SelectedTab,
    menus: HashMap<SelectedTab, Menu>,
}

impl AppWidget {
    pub async fn from_app(app: &App) -> Self {
        Self {
            selected_tab: app.selected_tab,
            menus: app.menus.read().await.clone(),
        }
    }
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
        let (tx, mut rx) = mpsc::channel(10);

        for tab in SelectedTab::iter() {
            let menus = self.menus.clone();
            let tx = tx.clone();
            tokio::spawn(async move {
                menus.write().await.insert(
                    tab,
                    Menu::new(
                        (Utc::now() + CDuration::days(tab as i64)).date_naive(),
                        &[Mensa::Academica, Mensa::Forum],
                    )
                    .await
                    .unwrap(),
                );
                // tokio::time::sleep(Duration::from_millis(500)).await;
                let _ = tx.send(()).await;
            });
        }

        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        while !self.exit {
            let widget = AppWidget::from_app(self).await;
            terminal.draw(|frame| widget.render_frame(frame))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            self.handle_events(timeout)?;

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }

            if rx.try_recv().is_ok() {
                // Handle any completed async tasks
            }
        }
        Ok(())
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn prev_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
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
            KeyCode::Left => self.prev_tab(),
            KeyCode::Right => self.next_tab(),
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

impl AppWidget {
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

        let details_block = Block::default().title("Details").borders(Borders::ALL);

        Paragraph::new(format!("Hello, World {}!", tab as i64))
            .block(details_block)
            .render(details, buf);

        let text = format!("{:?}", self.menus.get(&tab));

        Paragraph::new(text)
            .block(Block::default().title("Overview").borders(Borders::ALL))
            .render(overview, buf);
    }
}

impl Widget for &AppWidget {
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
    Line::raw("◄ ► to change tab | Press q to quit")
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
