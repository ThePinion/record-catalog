// use crossterm::event::{self, KeyCode};

// use crate::models::{
//     app::{AppPages, Navigation},
//     error::Result,
//     home::Home,
//     list::StatefulList,
//     record_detail::RecordDetail,
// };

// use super::inputer::CustomEvent;

// // impl InputReceiver for Home<'_> {
// //     fn input(&mut self, event: CustomEvent<event::KeyEvent>) -> Result<Navigation> {
// //         match event {
// //             CustomEvent::Input(key_event) => match key_event.code {
// //                 KeyCode::Down => self.search_results.next(),
// //                 KeyCode::Up => self.search_results.previous(),
// //                 KeyCode::Right => return self.select_release(),
// //                 KeyCode::Esc => return Ok(Navigation::QuitInput),
// //                 KeyCode::Enter => return self.search(),
// //                 KeyCode::Char('+') => {}
// //                 _ => {
// //                     self.search_textarea.input_without_shortcuts(key_event);
// //                 }
// //             },
// //             CustomEvent::Tick => {}
// //         };
// //         Ok(Navigation::DoNotihing)
// //     }
// // }
