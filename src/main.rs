#![feature(type_alias_enum_variants)]
use azul::prelude::*;

mod layout;
mod state;
use self::state::model::MeshApp;

const CSS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/style.css"));

fn main() {
    //let app = App::new(state::CounterApplication::default(), AppConfig::default());
    let app = App::new(MeshApp::Uninitialized, AppConfig::default());
    let css = css::from_str(CSS).expect("Unable to load CSS");
    let mut creation_options = WindowCreateOptions::default();
    creation_options.state.title = "Test App".to_owned();
    app.run(Window::new(creation_options, css).unwrap())
        .unwrap();
}
