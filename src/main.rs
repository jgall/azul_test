#![feature(type_alias_enum_variants)]
use azul::prelude::*;

mod layout;
mod state;
mod widgets;
use self::state::model::{MeshApp, Uninitialized};

const CUSTOM_CSS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/style.css"));

fn main() {
    let css = css::from_str(&vec![CUSTOM_CSS, widgets::dropdown::CSS].join(""))
        .expect("Unable to load CSS");
    //let app = App::new(state::CounterApplication::default(), AppConfig::default());
    let app = App::new(
        MeshApp::Uninitialized(Uninitialized {}),
        AppConfig::default(),
    );
    let mut creation_options = WindowCreateOptions::default();
    creation_options.state.title = "Test App".to_owned();
    app.run(Window::new(creation_options, css).unwrap())
        .unwrap();
}
