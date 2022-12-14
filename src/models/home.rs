use std::vec;

use tui::{
    style::Style,
    widgets::{Block, Borders},
};
use tui_textarea::TextArea;

use crate::discogs::DiscogsClient;

use super::{
    app::{AppNode, Navigation},
    list::StatefulList,
    query::DiscogsSearchResultRelease,
};

pub struct Home<'a> {
    pub search_textarea: TextArea<'a>,
    pub discogs_client: DiscogsClient,
    pub search_results: StatefulList<DiscogsSearchResultRelease>,
}

impl Home<'_> {
    pub fn new() -> Self {
        let discogs_client: DiscogsClient =
            DiscogsClient::new("gqvzVtgoghLkXbwsvkyXgmdoVeLZSebShZFpORVx");

        let search_results = vec![];

        let mut home = Home {
            search_textarea: TextArea::default(),
            discogs_client: discogs_client,
            search_results: StatefulList::with_items(search_results),
        };

        home.search_textarea.set_cursor_line_style(Style::default());
        home.search_textarea.set_block(
            Block::default()
                .title("Discogs Query")
                .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT),
        );
        home
    }
}

impl AppNode for Home<'_> {
    fn navigation(&mut self, _navigation: Navigation) {}
}
