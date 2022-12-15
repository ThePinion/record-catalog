use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode};

use crate::models::{
    app::{App, AppPage, AppPages, Navigation},
    error::Result,
    list::StatefulList,
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

impl App<'_> {
    pub fn input(&mut self, event: CustomEvent<event::KeyEvent>) -> Result<bool> {
        let navigation = self.handle_input(event)?;
        self.navigation(navigation)
    }

    fn navigation(&mut self, navigation: Navigation) -> Result<bool> {
        match navigation {
            Navigation::NavigatePage(page) => {
                self.active = page;
                match self.active {
                    AppPages::Search => {
                        let query = &self.main_input.lines()[0];
                        let items = self.database.search(&query);
                        self.search.list = StatefulList::with_items(items);
                    }
                    _ => {}
                };
            }
            // Navigation::ViewRecord(mut record_detail) => {
            //     let index: usize = (&AppPages::RecordDetail).into();
            //     record_detail.set_saved(match record_detail.record {
            //         Some(ref record) => self.database.contains(record),
            //         None => true,
            //     });
            //     self.active = index;
            //     self.list[index] = Box::new(record_detail);
            // }
            Navigation::EnterInput => {
                if self.active.show_input() {
                    self.input = true;
                }
            }

            Navigation::QuitInput => self.input = false,
            Navigation::Quit => return Ok(true),
            // Navigation::SaveRecord(record) => {
            //     let index: usize = (&AppPages::RecordDetail).into();
            //     self.list[index].navigation(Navigation::SaveRecord(record.clone()));
            //     self.database.add(record)?;
            // }
            Navigation::WebSearch => {
                self.search()?;
            }
            Navigation::Combined(vector) => {
                let results: Vec<_> = vector
                    .into_iter()
                    .map(|n| self.navigation(n).unwrap())
                    .collect();
                return Ok(results.contains(&true));
            }
            _ => {}
        }

        Ok(false)
    }

    fn handle_input(&mut self, event: CustomEvent<event::KeyEvent>) -> Result<Navigation> {
        if self.input {
            match event {
                CustomEvent::Input(key_event) => match key_event.code {
                    KeyCode::Esc => return Ok(Navigation::QuitInput),
                    KeyCode::Enter => {
                        return Ok(Navigation::Combined(vec![
                            Navigation::QuitInput,
                            Navigation::WebSearch,
                        ]))
                    }
                    _ => {
                        self.main_input.input_without_shortcuts(key_event);
                    }
                },
                _ => {}
            };

            return Ok(Navigation::DoNotihing);
        };

        Ok(match event {
            CustomEvent::Input(key_event) => match key_event.code {
                KeyCode::Char('h') => Navigation::NavigatePage(AppPages::Home),
                KeyCode::Char('w') => Navigation::NavigatePage(AppPages::WebSearch),
                KeyCode::Char('s') => Navigation::NavigatePage(AppPages::Search),
                KeyCode::Char('i') => Navigation::EnterInput,
                KeyCode::Char('q') => Navigation::Quit,

                c => return self.handle_page_specific_input(c),
            },

            CustomEvent::Tick => Navigation::DoNotihing,
        })
    }

    fn handle_page_specific_input(&mut self, code: KeyCode) -> Result<Navigation> {
        match self.active {
            AppPages::Home => Ok(Navigation::DoNotihing),
            AppPages::Search => Ok(Navigation::DoNotihing),
            AppPages::WebSearch => self.handle_web_search_input(code),
        }
    }

    fn handle_web_search_input(&mut self, code: KeyCode) -> Result<Navigation> {
        Ok(match code {
            KeyCode::Up => {
                self.query_results.previous();

                Navigation::DoNotihing
            }
            KeyCode::Down => {
                self.query_results.next();

                Navigation::DoNotihing
            }
            KeyCode::Enter => {
                self.search.selected = match self.select_release_from_web_search() {
                    Ok(r) => Some(r),
                    Err(_) => None,
                };

                Navigation::NavigatePage(AppPages::Search)
            }
            _ => Navigation::DoNotihing,
        })
    }
}
