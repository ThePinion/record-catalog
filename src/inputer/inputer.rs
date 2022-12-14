use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode};

use crate::models::{
    app::{App, AppPages, Navigation},
    error::Result,
};

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

pub trait InputReceiver {
    fn input(&mut self, event: CustomEvent<event::KeyEvent>) -> Result<Navigation>;
}
impl InputReceiver for App {
    fn input(&mut self, event: CustomEvent<event::KeyEvent>) -> Result<Navigation> {
        let navigation = self.handle_input(event)?;

        match navigation {
            Navigation::NavigateIndex(index) => self.active = index,
            Navigation::ViewRecord(mut record_detail) => {
                let index: usize = (&AppPages::RecordDetail).into();
                record_detail.set_saved(match record_detail.record {
                    Some(ref record) => self.database.contains(record),
                    None => true,
                });
                self.active = index;
                self.list[index] = Box::new(record_detail);
            }
            Navigation::EnterInput => self.input = true,
            Navigation::QuitInput => self.input = false,
            Navigation::Quit => return Ok(Navigation::Quit),
            Navigation::SaveRecord(record) => {
                self.database.add(record)?;
            }
            _ => {}
        }

        Ok(Navigation::DoNotihing)
    }
}

impl App {
    fn handle_input(&mut self, event: CustomEvent<event::KeyEvent>) -> Result<Navigation> {
        if self.input {
            return self.list[self.active].input(event);
        };

        match event {
            CustomEvent::Input(key_event) => {
                let bar = self.titles.iter().find(|p| {
                    KeyCode::Char(p.name.chars().nth(0).unwrap().to_ascii_lowercase())
                        == key_event.code
                });
                match bar {
                    Some(bar) => return Ok(Navigation::NavigateIndex(bar.position)),
                    None => match key_event.code {
                        KeyCode::Char('q') => return Ok(Navigation::Quit),
                        KeyCode::Char('i') => return Ok(Navigation::EnterInput),
                        KeyCode::Up
                        | KeyCode::Down
                        | KeyCode::Left
                        | KeyCode::Right
                        | KeyCode::Char('+') => {
                            return self.list[self.active].as_mut().input(event)
                        }

                        _ => {}
                    },
                }
            }
            CustomEvent::Tick => {}
        };

        Ok(Navigation::DoNotihing)
    }
}
