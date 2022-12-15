use crate::models::app::{AppPage, AppPages};

use super::super::models::error::Result;
use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};

use super::super::models::app::App;

impl App<'_> {
    pub fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        terminal.draw(|rect| {
            let chunks = self.layout_chunks(rect);
            self.render_menu(rect, chunks[0]);
            self.render_main(rect, chunks[1]);
            self.render_message(rect, chunks[2]);
        })?;
        Ok(())
    }

    fn render_menu(&self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let mut menu: Vec<_> = self
            .pages
            .iter()
            .map(|t| render_title(&t.get_title()))
            .collect();
        menu.push(render_title("Quit"));
        let temp = self.database.data.len().to_string();
        menu.push(render_title(temp.as_str()));

        // let input = self.input.to_string();
        // menu.push(render_title(&input));q

        let tabs = Tabs::new(menu)
            .select(self.active.get_number())
            .block(Block::default().title("Menu").borders(Borders::ALL))
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
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(size);
        chunks
    }

    fn render_main(&mut self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let main_area: Rect;
        if self.active.show_input() {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(2)].as_ref())
                .split(area);

            self.main_input.set_cursor_line_style(Style::default());

            let border_style = match self.input {
                true => Style::default()
                    .remove_modifier(Modifier::UNDERLINED)
                    .fg(Color::Yellow),
                false => Style::default(),
            };

            self.main_input.set_block(
                Block::default()
                    .title("Search")
                    .borders(Borders::ALL)
                    .border_style(border_style),
            );
            rect.render_widget(self.main_input.widget(), chunks[0]);

            main_area = chunks[1];
        } else {
            main_area = area;
        }

        self.render_main_area(rect, main_area)
    }

    fn render_main_area(&mut self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        match self.active {
            AppPages::Home => self.render_home(rect, area),
            AppPages::WebSearch => self.render_query_list(rect, area),
            AppPages::Search => self.render_search_page(rect, area),
        }
    }

    fn render_home(&mut self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
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

        rect.render_widget(home, area)
    }

    fn render_query_list(&mut self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let query_list = List::new(
            self.query_results
                .items
                .iter()
                .map(|r| {
                    ListItem::new(
                        match self.database.contains_id(r.id) {
                            true => " ✅ ",
                            false => " ❌ ",
                        }
                        .to_owned()
                            + &r.title.to_owned()
                            + &r.result_type
                            + &r.format
                                .iter()
                                .fold(" # ".to_string(), |a, f| a + f + " | ")
                            + &format!("{}", r.id),
                    )
                })
                .collect::<Vec<_>>(),
        )
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

        rect.render_stateful_widget(query_list, area, &mut self.query_results.state);
    }

    fn render_search_page(&mut self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(area);

        let query_list = List::new(
            self.search
                .list
                .items
                .iter()
                .map(|r| {
                    ListItem::new(
                        r.title.to_owned()
                            + &r.formats
                                .iter()
                                .fold(" # ".to_string(), |a, f| a + &f.name + " | ")
                            + &format!("{}", r.id),
                    )
                })
                .collect::<Vec<_>>(),
        )
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

        let contents: String;
        let title: String;
        if let Some(r) = &self.search.selected {
            title = r.title.clone()
                + " "
                + match self.database.contains(&r) {
                    true => " ✅ ",
                    false => " ❌ ",
                };
            contents = format!("{:#?}", r);
        } else {
            title = "".to_string();
            contents = "No record selected".to_string();
        };

        let detail = Paragraph::new(contents).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title(title.as_str())
                .border_type(BorderType::Plain),
        );

        rect.render_stateful_widget(query_list, chunks[0], &mut self.search.list.state);
        rect.render_widget(detail, chunks[1]);
    }

    fn render_message(&mut self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let message = Paragraph::new(vec![Spans::from(vec![Span::raw(self.message_box.clone())])])
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Message")
                    .border_type(BorderType::Plain),
            );

        rect.render_widget(message, area)
    }
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
