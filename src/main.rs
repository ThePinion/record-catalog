mod discogs;
mod gui;
mod inputer;
mod models;
use inputer::inputer::InputReceiver;
use models::{
    app::{App, Navigation},
    error::Result,
};

fn main() -> Result<()> {
    let receiver = inputer::inputer::start();

    let mut terminal = gui::terminal::start()?;

    let mut app = App::new();

    loop {
        app.render(&mut terminal)?;
        match app.input(receiver.recv()?)? {
            Navigation::Quit => break,
            _ => {}
        }
    }

    gui::terminal::end(&mut terminal)?;

    Ok(())
}
