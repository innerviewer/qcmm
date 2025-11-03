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

    // let metadata = qcm::Metadata {
    //     created_at: "2025-10-22".to_string().into(),
    //     author: "innerviewer".to_string().into(),
    // };

    // let qcm_file = qcm::QCMFile {
    //     version: 1,
    //     metadata: Some(metadata),
    //     qcm: test_qcm,
    // };

    // serialization::serialize_qcm(
    //     std::env::current_dir().unwrap().join("test.toml").as_path(),
    //     &qcm_file,
    // );

    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);

    ratatui::restore();
    Ok(app_result?)
}
