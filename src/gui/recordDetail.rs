use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::models::recordDetail::RecordDetail;

use super::render::Renderable;

impl Renderable for RecordDetail {
    fn render_main_block<'a>(&mut self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(2), Constraint::Length(2)].as_ref())
            .split(area);

        let title: &str = match &self.record {
            Some(r) => &r.title,
            None => "Empty",
        };
        let main = Paragraph::new(vec![Spans::from(vec![Span::raw(title)])])
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Record Detail")
                    .border_type(BorderType::Plain),
            );

        let message = Paragraph::new(vec![Spans::from(vec![Span::raw(&self.message)])])
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Message")
                    .border_type(BorderType::Plain),
            );

        rect.render_widget(main, chunks[0]);

        rect.render_widget(message, chunks[1]);
    }
}
