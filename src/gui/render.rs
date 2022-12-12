use super::super::models::error::Result;
use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Frame, Terminal,
};

use super::super::models::app::{App, AppPage, Home};

impl App<'_> {
    pub fn render(&self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        terminal.draw(|rect| {
            let chunks = self.layout_chunks(rect);
            self.render_menu(rect, chunks[0]);
            self.active().render_main_block(rect, chunks[1]);
        })?;
        Ok(())
    }

    fn render_menu(&self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let menu = self.list.iter().map(|t| t.render_title()).collect();

        let tabs = Tabs::new(menu)
            .select(self.active.into())
            .block(
                Block::default()
                    .title("Menu")
                    .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(Span::raw("|"));

        rect.render_widget(tabs, area);
    }

    fn layout_chunks(&self, rect: &mut Frame<CrosstermBackend<Stdout>>) -> Vec<Rect> {
        let size = rect.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(2), Constraint::Min(2)].as_ref())
            .split(size);
        chunks
    }
}

pub trait Renderable {
    fn render_title(&self) -> Spans;
    fn render_main_block<'a>(&self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect);
}

impl Renderable for AppPage<'_> {
    fn render_title(&self) -> Spans {
        let t_name = self.to_string();
        let (first, rest) = t_name.split_at(1);

        Spans::from(vec![
            Span::styled(
                first.to_owned(),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::UNDERLINED),
            ),
            Span::styled(rest.to_owned(), Style::default().fg(Color::White)),
        ])
    }

    fn render_main_block(&self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        match &self {
            AppPage::Home(h) => h.render_home(rect, area),
            _ => (),
        };
    }
}

impl Home<'_> {
    fn render_home<'a>(&self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(2), Constraint::Min(2)].as_ref())
            .split(area);

        let home = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Welcome to")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(
                "RecordCatalog-CLI",
                Style::default().fg(Color::LightBlue),
            )]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Navigate using the keyboard.")]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Home")
                .border_type(BorderType::Plain),
        );

        rect.render_widget(self.search_textarea.widget(), chunks[0]);
        rect.render_widget(home, chunks[1]);
    }
}
