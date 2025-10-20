use color_eyre::Result;

mod app;
mod events;
mod ui;

use crate::app::App;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);

    ratatui::restore();
    Ok(app_result?)
}

