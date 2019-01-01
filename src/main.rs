use azul::css::CssProperty;
use azul::css::{LayoutLeft, LayoutTop};
use azul::{prelude::*, widgets::button::Button};
use std::sync::{Arc, RwLock};

mod layout;
mod state;
use self::state::{mesh, model::MeshApp};

const CSS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/style.css"));

fn main() {
    //let app = App::new(state::CounterApplication::default(), AppConfig::default());
    let app = App::new(
        MeshApp {
            mesh: mesh::Mesh::new_grid(15.0, 20.0, 20.0, 20, 20),
            dragging: None,
        },
        AppConfig::default(),
    );
    let css = css::from_str(CSS).expect("Unable to load CSS");
    let mut creation_options = WindowCreateOptions::default();
    creation_options.state.title = "Test App".to_owned();
    app.run(Window::new(creation_options, css).unwrap())
        .unwrap();
}
