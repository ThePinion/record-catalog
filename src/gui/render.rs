use crate::models::{
    app::{AppPage, AppPages},
    item_holder::StatefulItem,
    list::StatefulList,
    record::Record,
};

use super::super::models::error::Result;
use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs, Wrap},
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

            let border_style = match self.is_main_input {
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
                            true => " ✅   ",
                            false => " ❌   ",
                        }
                        .to_owned()
                            + &r.title.to_owned()
                            + "    "
                            + &r.format.iter().fold("".to_string(), |a, f| a + f + " | "),
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
            .constraints(self.get_layout_chunks().as_ref())
            .split(area);

        self.render_search_list(rect, chunks[0]);
        if let Some(item_holder) = self.search.list.selected_mut() {
            Self::render_item_holder_items_list(&mut item_holder.list, rect, chunks[1]);
            match item_holder.list.selected_mut() {
                Some(item) => {
                    let border_style = match self.is_side_input {
                        true => Style::default()
                            .remove_modifier(Modifier::UNDERLINED)
                            .fg(Color::Yellow),
                        false => Style::default(),
                    };
                    self.side_input.set_block(
                        Block::default()
                            .title("Message")
                            .borders(Borders::ALL)
                            .border_style(border_style),
                    );
                    let small_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Min(4), Constraint::Length(3)].as_ref())
                        .split(chunks[2]);
                    rect.render_widget(self.side_input.widget(), small_chunks[1]);
                    Self::render_item_detail(&item.clone(), rect, small_chunks[0])
                }
                None => Self::render_record_detail(
                    &item_holder.record,
                    item_holder.detail_offset,
                    rect,
                    chunks[2],
                ),
            }
        }
    }

    fn get_layout_chunks(&mut self) -> [Constraint; 3] {
        match self
            .search
            .list
            .selected_mut()
            .map_or(None, |ih| ih.list.selected())
        {
            Some(_) => [
                Constraint::Percentage(25),
                Constraint::Percentage(30),
                Constraint::Percentage(40),
            ],
            None => [
                Constraint::Percentage(35),
                Constraint::Percentage(0),
                Constraint::Percentage(65),
            ],
        }
    }

    fn render_search_list(&mut self, rect: &mut Frame<CrosstermBackend<Stdout>>, area: Rect) {
        let query_list = List::new(
            self.search
                .list
                .items
                .iter()
                .map(|ih| ih.record.clone())
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
        rect.render_stateful_widget(query_list, area, &mut self.search.list.state);
    }

    fn render_record_detail(
        record: &Record,
        detail_offset: usize,
        rect: &mut Frame<CrosstermBackend<Stdout>>,
        area: Rect,
    ) {
        let spans: Vec<_> = record
            .get_lines()
            .into_iter()
            .map(|s| Spans::from(vec![Span::raw(""), Span::raw(s)]))
            .collect::<Vec<_>>()[detail_offset..]
            .into();

        let detail = Paragraph::new(spans).wrap(Wrap { trim: false }).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Record")
                .border_type(BorderType::Plain),
        );

        rect.render_widget(detail, area);
    }

    fn render_item_detail(
        stateful_item: &StatefulItem,
        rect: &mut Frame<CrosstermBackend<Stdout>>,
        area: Rect,
    ) {
        let spans: Vec<_> = stateful_item
            .item
            .events
            .iter()
            .map(|ie| Spans::from(vec![Span::raw("\n    "), Span::raw(format!("{:#?}", ie))]))
            .collect::<Vec<_>>()
            .into();

        let detail = Paragraph::new(spans).wrap(Wrap { trim: false }).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Item events")
                .border_type(BorderType::Plain),
        );
        rect.render_widget(detail, area);
    }

    fn render_item_holder_items_list(
        item_holder_list: &mut StatefulList<StatefulItem>,
        rect: &mut Frame<CrosstermBackend<Stdout>>,
        area: Rect,
    ) {
        let list = List::new(
            item_holder_list
                .items
                .iter()
                .map(|i| ListItem::new(format!("{}", i.item.id)))
                .collect::<Vec<_>>(),
        )
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );
        rect.render_stateful_widget(list, area, &mut item_holder_list.state);
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
