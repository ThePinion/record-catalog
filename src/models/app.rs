use tui_textarea::TextArea;

use crate::{database::Database, discogs::DiscogsClient};

use super::{
    error::Result, item_holder::ItemHolder, list::StatefulList, query::DiscogsSearchResultRelease,
    record::Record, settings::Settings,
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
    pub message_box: String,
    pub query_results: StatefulList<DiscogsSearchResultRelease>,
    pub search: Search,
}

impl App<'_> {
    pub fn new(settings: Settings) -> Result<Self> {
        let discogs_client: DiscogsClient = DiscogsClient::new(&settings.discogs_key);

        let search_results = vec![];

        let input = TextArea::default();

        Ok(App {
            pages: AppPages::iter().collect::<Vec<_>>(),
            active: AppPages::Home,
            input: false,
            database: Database::new(&settings.database_path)?,
            discogs_client: discogs_client,
            main_input: input,
            query_results: StatefulList::with_items(search_results),
            message_box: "".to_string(),
            search: Search::empty(),
        })
    }

    pub fn web_search(&mut self) -> Result<()> {
        let query = &self.main_input.lines()[0];
        let results = self.discogs_client.query(&query)?.get_releases();
        self.message_box = format!("Found {} results", results.len());
        self.query_results = StatefulList::with_items(results);
        return Ok(());
    }

    pub fn search(&mut self) -> Result<()> {
        let query = &self.main_input.lines()[0];
        let results = self.database.search(query);
        self.message_box = format!("Found {} results", results.len());
        self.search.list = StatefulList::with_items(results);
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
    InputSubmit,
    DoNotihing,
    QuitInput,
    EnterInput,
    Quit,
    Combined(Vec<Navigation>),
}

pub struct Search {
    pub list: StatefulList<ItemHolder>,
    pub selected: Option<ItemHolder>,
    pub is_saved: bool,
    pub detail_offset: usize,
}

impl Search {
    pub fn empty() -> Self {
        Search {
            list: StatefulList::with_items(vec![]),
            selected: None,
            is_saved: true,
            detail_offset: 0,
        }
    }
}

impl Into<usize> for AppPages {
    fn into(self) -> usize {
        self as usize
    }
}
