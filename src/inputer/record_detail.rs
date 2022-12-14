use crate::models::record_detail::RecordDetail;

use crossterm::event::{self, KeyCode};

use crate::models::{app::Navigation, error::Result};

use super::inputer::{CustomEvent, InputReceiver};

impl InputReceiver for RecordDetail {
    fn input(&mut self, event: CustomEvent<event::KeyEvent>) -> Result<Navigation> {
        match event {
            CustomEvent::Input(key_event) => match key_event.code {
                KeyCode::Right => {}
                KeyCode::Esc | KeyCode::Left => return Ok(self.back_instruction.clone()),
                _ => {}
            },
            CustomEvent::Tick => {}
        };
        Ok(Navigation::DoNotihing)
    }
}
