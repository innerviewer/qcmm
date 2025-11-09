#![allow(dead_code, unused_variables, unused_imports, unused_import_braces)]

use color_eyre::Result;
use std::env;

mod app;
mod events;
mod qcm;
mod renderable;
mod serialization;
mod ui;

use crate::app::App;

// TODO: Implement WidgetRef
// TODO: `cargo add directories` -> crate for handling data directories
// optimizations in Cargo.toml?
// tachyonfx?
fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);

    ratatui::restore();
    Ok(app_result?)
}
