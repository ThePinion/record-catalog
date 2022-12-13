mod discogs;
mod gui;
mod inputer;
mod models;
use inputer::inputer::InputReceiver;
use models::{app::App, error::Result};

fn main() -> Result<()> {
    let receiver = inputer::inputer::start();

    let mut terminal = gui::terminal::start()?;

    let mut app = App::new();

    loop {
        app.render(&mut terminal)?;
        match app.receive_input(receiver.recv()?)? {
            false => break,
            true => (),
        }
    }

    gui::terminal::end(&mut terminal)?;

    Ok(())
}
