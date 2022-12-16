mod database;
mod discogs;
mod gui;
mod inputer;
mod models;
use std::fs;

use models::{app::App, error::Result, record::Label, settings::Settings};

fn main() -> Result<()> {
    // return test();

    let receiver = inputer::inputer::start();

    let mut terminal = gui::terminal::start()?;

    let mut app = App::new(load_settings()?)?;

    loop {
        app.render(&mut terminal)?;
        if app.input(receiver.recv()?)? {
            break;
        }
    }

    gui::terminal::end(&mut terminal)?;

    Ok(())
}

fn load_settings() -> Result<Settings> {
    let string = fs::read_to_string("settings.json")?;
    Ok(serde_json::from_str::<Settings>(&string)?)
}

#[allow(dead_code)]
fn test() -> Result<()> {
    let original = Label {
        name: "Super Label".to_string(),
        catno: "Super duper".to_string(),
    };

    let cloned = original.clone();

    let serialized = serde_json::to_string(&original)?;
    let parsed = serde_json::from_str::<Label>(&serialized)?;

    println!("Cloned: {:?}", cloned == original);
    println!("Parsed: {:?}", parsed == original);

    Ok(())
}
