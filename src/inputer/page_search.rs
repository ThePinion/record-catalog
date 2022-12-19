use crossterm::event::KeyCode;

use crate::models::{
    app::{App, Navigation},
    error::Result,
    item_holder::ItemHolder,
};

impl App<'_> {
    pub fn handle_search_input(&mut self, code: KeyCode) -> Result<Navigation> {
        Ok(match code {
            KeyCode::Up => {
                self.search.list.previous();
                self.search_page_update_selected();
                Navigation::DoNotihing
            }
            KeyCode::Down => {
                self.search.list.next();
                self.search_page_update_selected();
                Navigation::DoNotihing
            }
            KeyCode::Insert | KeyCode::Char('+') => {
                if !self.search.is_saved {
                    match &self.search.selected {
                        Some(r) => {
                            self.database.add(r.record.clone())?;
                            self.search.is_saved = true;
                            self.message_box = "Record Saved".to_string();
                            return Ok(Navigation::InputSubmit);
                        }
                        None => {}
                    }
                } else {
                    self.message_box = "Record already saved".to_string();
                }
                Navigation::DoNotihing
            }
            KeyCode::Delete | KeyCode::Char('-') => {
                if self.search.is_saved {
                    match &self.search.selected {
                        Some(r) => {
                            self.database.remove(&r.record, 0)?;
                            self.search.is_saved = false;
                            self.message_box = "Record Deleted".to_string();
                            return Ok(Navigation::InputSubmit);
                        }
                        None => {}
                    }
                } else {
                    self.message_box = "Record not saved".to_string();
                }
                Navigation::DoNotihing
            }
            KeyCode::PageUp => {
                let offset = self.search.detail_offset;
                if offset > 0 {
                    self.search.detail_offset -= 1;
                }
                Navigation::DoNotihing
            }
            KeyCode::PageDown => {
                if let Some(r) = &self.search.selected {
                    let offset = self.search.detail_offset;
                    if offset < r.record.get_lines().len() - 1 {
                        self.search.detail_offset += 1;
                    }
                }

                Navigation::DoNotihing
            }
            KeyCode::Enter => Navigation::InputSubmit,
            _ => Navigation::DoNotihing,
        })
    }

    pub fn search_page_update_selected(&mut self) {
        if let Some(index) = self.search.list.state.selected() {
            self.search_page_set_selected(self.search.list.items[index].clone());
        }
    }

    pub fn search_page_set_selected(&mut self, holder: ItemHolder) {
        self.search.is_saved = self.database.contains(&holder.record);
        self.search.detail_offset = 0;
        self.search.selected = Some(holder);
    }
}
