#![allow(unused_variables, unused_imports, unused_imports_braces)]

use color_eyre::Result;

mod app;
mod events;
mod question;
mod ui;

use crate::app::App;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);

    ratatui::restore();
    Ok(app_result?)
}
