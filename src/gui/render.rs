use super::super::models::error::Result;
use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame, Terminal,
};

use super::super::models::app::App;

impl App {
    pub fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        terminal.draw(|rect| {
            let chunks = self.layout_chunks(rect);
            self.render_menu(rect, chunks[0]);
            self.list[self.active]
                .as_mut()
                .render_main_block(rect, chunks[1]);
        })?;
        Ok(())
    }

    fn render_menu(&self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let mut menu: Vec<_> = self.titles.iter().map(|t| render_title(&t.name)).collect();
        menu.push(render_title("Quit"));
        let temp = self.database.data.len().to_string();
        menu.push(render_title(temp.as_str()));

        // let input = self.input.to_string();
        // menu.push(render_title(&input));q

        let tabs = Tabs::new(menu)
            .select(self.active)
            .block(
                Block::default()
                    .title("Menu")
                    .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .remove_modifier(Modifier::UNDERLINED)
                    .fg(Color::Black)
                    .bg(Color::Yellow),
            )
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
    fn render_main_block<'a>(&mut self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect);
}

fn render_title(title: &str) -> Spans {
    let (first, rest) = title.split_at(1);

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
