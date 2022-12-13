use crate::{gui::render::Renderable, inputer::inputer::InputReceiver};

use super::{home::Home, record::Record, recordDetail::RecordDetail};

pub struct App {
    pub active: usize,
    pub list: Vec<Box<dyn AppNode>>,
    pub titles: Vec<TitleBar>,
    pub input: bool,
}

impl App {
    pub fn new() -> Self {
        let mut list: Vec<Box<dyn AppNode>> = vec![];
        list.push(Box::new(Home::new()));
        let titles = vec![TitleBar {
            name: "Home".to_string(),
            position: 0,
        }];
        list.push(Box::new(RecordDetail::empty(Navigation::NavigateIndex(0))));
        App {
            list: list,
            titles: titles,
            active: 0,
            input: false,
        }
    }
}

#[derive(Clone)]
pub enum Navigation {
    ViewRelease(Record),
    NavigateIndex(usize),
    DoNotihing,
    QuitInput,
    EnterInput,
    Quit,
}

pub trait AppNode: Renderable + InputReceiver {}

pub struct TitleBar {
    pub name: String,
    pub position: usize,
}
