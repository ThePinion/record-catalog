use crossterm::event::{self, KeyCode};

use crate::models::{app::Navigation, error::Result, home::Home, list::StatefulList};

use super::inputer::{CustomEvent, InputReceiver};

impl InputReceiver for Home<'_> {
    fn input(&mut self, event: CustomEvent<event::KeyEvent>) -> Result<Navigation> {
        match event {
            CustomEvent::Input(key_event) => match key_event.code {
                KeyCode::Down => self.search_results.next(),
                KeyCode::Up => self.search_results.previous(),
                KeyCode::Right => return self.select_release(),
                KeyCode::Esc => return Ok(Navigation::QuitInput),
                KeyCode::Enter => return self.search(),
                _ => {
                    self.search_textarea.input_without_shortcuts(key_event);
                }
            },
            CustomEvent::Tick => {}
        };
        Ok(Navigation::DoNotihing)
    }
}

impl<'a> Home<'a> {
    fn select_release(&mut self) -> Result<Navigation> {
        let index = self.search_results.state.selected();
        Ok(match index {
            Some(i) if i < self.search_results.items.len() => {
                let release = &self.search_results.items[i];
                let record = self.discogs_client.get_release(release.id)?;
                Navigation::ViewRelease(record)
            }
            _ => Navigation::DoNotihing,
        })
    }

    fn search(&mut self) -> Result<Navigation> {
        let query = &self.search_textarea.lines()[0];
        let results = self.discogs_client.query(&query)?.get_releases();
        self.search_results = StatefulList::with_items(results);
        return Ok(Navigation::QuitInput);
    }
}
