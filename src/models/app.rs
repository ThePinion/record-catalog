use tui::{
    style::Style,
    widgets::{Block, Borders},
};
use tui_textarea::TextArea;

use crate::{database::Database, discogs::DiscogsClient};

use super::{
    error::Result, list::StatefulList, query::DiscogsSearchResultRelease, record::Record,
    record_detail::RecordDetail,
};

use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Clone, PartialEq)]
pub enum AppPages {
    Home,
    Search,
    WebSearch,
}

pub trait AppPage {
    fn get_title(&self) -> &str;
    fn show_input(&self) -> bool;
    fn get_number(&self) -> usize;
}

impl AppPage for AppPages {
    fn get_title(&self) -> &str {
        match self {
            AppPages::Home => "Home",
            AppPages::WebSearch => "Web search",
            AppPages::Search => "Search",
        }
    }

    fn get_number(&self) -> usize {
        match self {
            AppPages::Home => 0,
            AppPages::Search => 1,
            AppPages::WebSearch => 2,
        }
    }

    fn show_input(&self) -> bool {
        match self {
            AppPages::Home => false,
            AppPages::WebSearch => true,
            AppPages::Search => true,
        }
    }
}

pub struct App<'a> {
    pub active: AppPages,
    pub pages: Vec<AppPages>,
    pub input: bool,
    pub database: Database,
    pub discogs_client: DiscogsClient,
    pub main_input: TextArea<'a>,
    pub message_box: &'a str,
    pub query_results: StatefulList<DiscogsSearchResultRelease>,
    pub search: Search,
}

impl App<'_> {
    pub fn new() -> Result<Self> {
        let discogs_client: DiscogsClient =
            DiscogsClient::new("gqvzVtgoghLkXbwsvkyXgmdoVeLZSebShZFpORVx");

        let search_results = vec![];

        let mut input = TextArea::default();

        Ok(App {
            pages: AppPages::iter().collect::<Vec<_>>(),
            active: AppPages::Home,
            input: false,
            database: Database::new("database.json")?,
            discogs_client: discogs_client,
            main_input: input,
            query_results: StatefulList::with_items(search_results),
            message_box: "",
            search: Search::empty(),
        })
    }

    pub fn search(&mut self) -> Result<()> {
        let query = &self.main_input.lines()[0];
        let results = self.discogs_client.query(&query)?.get_releases();
        self.query_results = StatefulList::with_items(results);
        self.query_results.next();
        return Ok(());
    }

    pub fn select_release_from_web_search(&mut self) -> Result<Record> {
        let index = self.query_results.state.selected();
        match index {
            Some(i) if i < self.query_results.items.len() => {
                let release = &self.query_results.items[i];
                let record = self.discogs_client.get_release(release.id);

                return record;
            }
            _ => Err("No release")?,
        }
    }
}

#[derive(Clone)]
pub enum Navigation {
    NavigatePage(AppPages),
    WebSearch,
    ViewRecord,
    SaveRecord,
    DoNotihing,
    QuitInput,
    EnterInput,
    Quit,
    Combined(Vec<Navigation>),
}

pub struct Search {
    pub list: StatefulList<Record>,
    pub selected: Option<Record>,
    pub is_saved: bool,
}

impl Search {
    pub fn empty() -> Self {
        Search {
            list: StatefulList::with_items(vec![]),
            selected: None,
            is_saved: true,
        }
    }
}

// #[derive(Copy, Clone)]
// pub enum AppPages {
//     Home,
//     RecordDetail,
// }

impl Into<usize> for AppPages {
    fn into(self) -> usize {
        self as usize
    }
}

pub struct TitleBar {
    pub name: String,
    pub position: usize,
}
