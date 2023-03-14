use crossterm::event::KeyCode;

use crate::models::{
    app::{App, Navigation},
    error::Result,
    item_holder::ItemEventType,
};

impl App<'_> {
    pub fn handle_search_input(&mut self, code: KeyCode) -> Result<Navigation> {
        Ok(match code {
            KeyCode::Up => {
                match self.search.list.selected_mut() {
                    Some(item_holder) if matches!(item_holder.list.selected(), Some(_)) => {
                        item_holder.list.previous();
                        return Ok(Navigation::QuitSideInput);
                    }
                    _ => self.search.list.previous(),
                }
                Navigation::DoNotihing
            }
            KeyCode::Down => {
                match self.search.list.selected_mut() {
                    Some(item_holder) if matches!(item_holder.list.selected(), Some(_)) => {
                        item_holder.list.next();
                        return Ok(Navigation::QuitSideInput);
                    }
                    _ => self.search.list.next(),
                }
                Navigation::DoNotihing
            }
            KeyCode::Right => {
                match &mut self.search.list.selected_mut() {
                    Some(selected) => selected.list.next(),
                    None => (),
                }
                Navigation::DoNotihing
            }
            KeyCode::Left => {
                match &mut self.search.list.selected_mut() {
                    Some(selected) => selected.list.unselect(),
                    None => (),
                }
                Navigation::DoNotihing
            }
            KeyCode::Char('+') => {
                match &mut self.search.list.selected_mut() {
                    Some(r) => {
                        let mut updated = self.database.add(r.record.clone())?.to_stateful();
                        updated.list.previous();
                        updated.list.previous();
                        self.message_box = "Record Saved".to_string();
                        self.search.list.update_selected(updated);
                        return Ok(Navigation::DoNotihing);
                    }
                    None => {}
                }

                Navigation::DoNotihing
            }
            KeyCode::Char('_') => {
                match self.search.list.selected_mut() {
                    Some(selected) if matches!(selected.list.selected(), Some(_)) => {
                        let index = selected.list.state.selected().unwrap();
                        match self.database.remove_holder_item(&selected.record, index) {
                            Ok(_) => {
                                selected.list.remove_at_index(index);
                                self.message_box = "Item removed".to_string();
                            }
                            Err(_) => self.message_box = "Couldn't remove item".to_string(),
                        }
                    }
                    _ => self.message_box = "Item not selected".to_string(),
                }
                Navigation::DoNotihing
            }
            KeyCode::Char('-') => {
                match self.search.list.selected_mut() {
                    Some(selected) => {
                        if selected.list.items.is_empty() {
                            self.database.remove_holder(&selected.record)?;
                            self.message_box = "Record removed".to_string();
                        } else {
                            self.message_box = "Items list not empty".to_string();
                        }
                    }
                    _ => self.message_box = "Record not selected".to_string(),
                }
                Navigation::DoNotihing
            }
            KeyCode::PageUp => {
                if let Some(mut selected) = self.search.list.selected_mut() {
                    if selected.detail_offset > 0 {
                        selected.detail_offset -= 1;
                    }
                }
                Navigation::DoNotihing
            }
            KeyCode::PageDown => {
                if let Some(mut selected) = self.search.list.selected_mut() {
                    if selected.detail_offset < selected.record.get_lines().len() - 1 {
                        selected.detail_offset += 1;
                    }
                }

                Navigation::DoNotihing
            }
            KeyCode::Char('m') => {
                if let Some(item) = &mut self.search.get_selected_item_mut() {
                    item.input = Some(ItemEventType::Message);
                    Navigation::EnterSideInput
                } else {
                    Navigation::DoNotihing
                }
            }
            KeyCode::Char('l') => {
                if let Some(item) = &mut self.search.get_selected_item_mut() {
                    item.input = Some(ItemEventType::Lent);
                    Navigation::EnterSideInput
                } else {
                    Navigation::DoNotihing
                }
            }
            KeyCode::Enter => Navigation::InputSubmit,
            _ => Navigation::DoNotihing,
        })
    }

    // pub fn search_page_update_selected(&mut self) {
    //     if let Some(index) = self.search.list.state.selected() {
    //         self.search_page_set_selected(self.search.list.items[index].clone());
    //     }
    // }

    // pub fn search_page_set_selected(&mut self, holder: ItemHolder) {
    //     self.search.is_saved = self.database.contains(&holder.record);
    //     self.search.detail_offset = 0;
    //     self.search.selected = Some(holder.to_stateful());
    // }
}
