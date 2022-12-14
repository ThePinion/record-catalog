use crate::models::record_detail::RecordDetail;

use crossterm::event::{self, KeyCode};

use crate::models::{app::Navigation, error::Result};

use super::inputer::{CustomEvent, InputReceiver};

impl InputReceiver for RecordDetail {
    fn input(&mut self, event: CustomEvent<event::KeyEvent>) -> Result<Navigation> {
        match event {
            CustomEvent::Input(key_event) => match key_event.code {
                KeyCode::Right => {}
                KeyCode::Esc | KeyCode::Left => return Ok(self.back_instruction.as_mut().clone()),
                KeyCode::Char('+') => {
                    if !self.is_saved {
                        match &self.record {
                            Some(r) => return Ok(Navigation::SaveRecord(r.clone())),
                            None => {}
                        }
                    }
                }
                _ => {}
            },
            CustomEvent::Tick => {}
        };
        Ok(Navigation::DoNotihing)
    }
}
