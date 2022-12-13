use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::models::home::Home;

use super::render::Renderable;

impl Renderable for Home<'_> {
    fn render_main_block<'a>(&mut self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
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

        let home_list = List::new(
            self.search_results
                .items
                .iter()
                .map(|r| ListItem::new(r.title.to_owned()))
                .collect::<Vec<_>>(),
        )
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" + ");

        rect.render_widget(self.search_textarea.widget(), chunks[0]);

        if self.search_results.items.len() > 0 {
            rect.render_stateful_widget(home_list, chunks[1], &mut self.search_results.state);
        } else {
            rect.render_widget(home, chunks[1]);
        }
    }
}
