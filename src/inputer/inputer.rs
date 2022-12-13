use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode};

use crate::models::{app::App, error::Result};

pub enum CustomEvent<I> {
    Input(I),
    Tick,
}

pub fn start() -> mpsc::Receiver<CustomEvent<event::KeyEvent>> {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let Event::Key(key) = event::read().expect("can read events") {
                    tx.send(CustomEvent::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(CustomEvent::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
    rx
}

pub trait InputReceiver<T> {
    fn receive_input(&mut self, event: CustomEvent<event::KeyEvent>) -> Result<T>;
}
impl InputReceiver<bool> for App {
    fn receive_input(&mut self, event: CustomEvent<event::KeyEvent>) -> Result<bool> {
        if self.input {
            let get_input = self.list[self.active].node.receive_input(event)?;
            self.input = !get_input.relinquish_control;
            return Ok(true);
        }

        match event {
            CustomEvent::Input(key_event) => {
                let pos = self.list.iter().position(|p| {
                    KeyCode::Char(p.title.chars().nth(0).unwrap().to_ascii_lowercase())
                        == key_event.code
                });
                match pos {
                    Some(index) => self.active = index,
                    None => match key_event.code {
                        KeyCode::Char('q') => return Ok(false),
                        KeyCode::Char('i') => self.input = true,
                        KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                            self.list[self.active].node.as_mut().receive_input(event)?;
                        }
                        _ => {}
                    },
                }
            }
            CustomEvent::Tick => {}
        };
        Ok(true)
    }
}
