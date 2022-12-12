use std::fmt;

use tui::style::Style;
use tui_textarea::TextArea;

pub struct App<'a> {
    pub active: usize,
    pub list: Vec<AppPage<'a>>,
    pub input: bool,
}

impl App<'_> {
    pub fn new() -> Self {
        let list = vec![AppPage::Home(Home::new()), AppPage::List, AppPage::Quit];
        App {
            list: list,
            active: 0,
            input: false,
        }
    }

    pub fn active(&self) -> &AppPage {
        &self.list[self.active]
    }
}

#[derive(Clone)]
pub enum AppPage<'a> {
    Home(Home<'a>),
    List,
    Quit,
}

impl From<&AppPage<'_>> for usize {
    fn from(input: &AppPage) -> usize {
        match input {
            &AppPage::Home(_) => 0,
            &AppPage::List => 1,
            &AppPage::Quit => 2,
        }
    }
}

impl fmt::Display for AppPage<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AppPage::Home(_) => write!(f, "{}", "Home"),
            AppPage::List => write!(f, "{}", "List"),
            AppPage::Quit => write!(f, "{}", "Quit"),
        }
    }
}

#[derive(Clone)]
pub struct Home<'a> {
    pub search_textarea: TextArea<'a>,
}

impl Home<'_> {
    pub fn new() -> Self {
        let mut home = Home {
            search_textarea: TextArea::default(),
        };
        home.search_textarea.set_cursor_line_style(Style::default());
        home
    }
}
