use crossterm::event::{self, KeyCode};

use crate::models::{app::NavigationResult, home::Home, list::StatefulList};

use super::inputer::{CustomEvent, InputReceiver};

impl InputReceiver<NavigationResult> for Home<'_> {
    fn receive_input(
        &mut self,
        event: CustomEvent<event::KeyEvent>,
    ) -> crate::models::error::Result<NavigationResult> {
        match event {
            CustomEvent::Input(key_event) => match key_event.code {
                KeyCode::Down => self.search_results.next(),
                KeyCode::Up => self.search_results.previous(),
                KeyCode::Esc => return Ok(NavigationResult::new(true)),
                KeyCode::Enter => {
                    let query = &self.search_textarea.lines()[0];
                    let results = self.discogs_client.query(&query).unwrap().get_releases();
                    self.search_results = StatefulList::with_items(results);
                    return Ok(NavigationResult::new(true));
                }
                _ => {
                    self.search_textarea.input_without_shortcuts(key_event);
                }
            },
            CustomEvent::Tick => {}
        };
        Ok(NavigationResult::new(false))
    }
}
