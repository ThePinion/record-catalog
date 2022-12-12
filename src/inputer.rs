use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode};

use crate::models::{
    app::App,
    error::{AppError, Result},
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

pub trait Receiver {
    fn receive(
        &mut self,
        receiver: &mut mpsc::Receiver<CustomEvent<event::KeyEvent>>,
    ) -> Result<()>;
}
impl Receiver for App<'_> {
    fn receive(
        &mut self,
        receiver: &mut mpsc::Receiver<CustomEvent<event::KeyEvent>>,
    ) -> Result<()> {
        match receiver.recv()? {
            CustomEvent::Input(event) => match event.code {
                KeyCode::Char('q') => return Err(Box::new(AppError("QUIT".to_owned()))),
                KeyCode::Char('h') => self.active = 0,
                KeyCode::Char('l') => self.active = 1,
                _ => {}
            },
            CustomEvent::Tick => {}
        };
        Ok(())
    }
}
