mod discogs;
mod gui;
mod inputer;
mod models;
use discogs::DiscogsClient;
use inputer::Receiver;
use models::{app::App, error::Result};

fn main() -> Result<()> {
    let discogs_client: DiscogsClient =
        DiscogsClient::new("gqvzVtgoghLkXbwsvkyXgmdoVeLZSebShZFpORVx");

    // let releases = discogs_client.query("Nirvana").get_releases();
    // println!("{:#?}", &releases)
    let release = discogs_client.get_release(11160944);
    // println!("{:#?}", &release.unwrap())

    let mut receiver = inputer::start();

    let mut terminal = gui::terminal::start()?;

    let mut app = App::new();

    loop {
        app.render(&mut terminal)?;
        match app.receive(&mut receiver) {
            Ok(_) => (),
            Err(_) => break,
        }
    }

    gui::terminal::end(&mut terminal)?;

    Ok(())
}
