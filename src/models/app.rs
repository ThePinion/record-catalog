use crate::{gui::render::Renderable, inputer::inputer::InputReceiver};

use super::home::Home;

pub struct App {
    pub active: usize,
    pub list: Vec<AppPage>,
    pub input: bool,
}

impl App {
    pub fn new() -> Self {
        let mut list: Vec<AppPage> = vec![];
        list.push(AppPage {
            node: Box::new(Home::new()),
            title: "Home".to_string(),
        });
        list.push(AppPage {
            node: Box::new(Home::new()),
            title: "Dome".to_string(),
        });
        App {
            list: list,
            active: 0,
            input: false,
        }
    }
}

pub struct AppPage {
    pub node: Box<dyn AppNode<NavigationResult>>,
    pub title: String,
}

pub struct NavigationResult {
    pub relinquish_control: bool,
}

impl NavigationResult {
    pub fn new(relinquish_control: bool) -> Self {
        NavigationResult { relinquish_control }
    }
}

pub trait AppNode<T>: Renderable + InputReceiver<T> {}

// impl From<&AppPage<'_>> for usize {
//     fn from(input: &AppPage) -> usize {
//         match input {
//             &AppPage::Home(_) => 0,
//             &AppPage::List => 1,
//             &AppPage::Quit => 2,
//         }
//     }
// }

// impl fmt::Display for AppPage<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match &self {
//             AppPage::Home(_) => write!(f, "{}", "Home"),
//             AppPage::List => write!(f, "{}", "List"),
//             AppPage::Quit => write!(f, "{}", "Quit"),
//         }
//     }
// }
