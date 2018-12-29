use azul::css::CssProperty;
use azul::css::{LayoutLeft, LayoutTop};
use azul::{prelude::*, widgets::button::Button};
use std::sync::{Arc, RwLock};

mod mesh;

const CSS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/style.css"));

struct MeshApp {
    mesh: mesh::Mesh,
    dragging: Option<Arc<RwLock<mesh::Particle>>>,
}

impl Layout for MeshApp {
    fn layout(&self, _info: WindowInfo<Self>) -> Dom<Self> {
        let mut dom = Dom::new(NodeType::Div);
        for p in &self.mesh.particles {
            let p = p.read().unwrap();
            dom.add_child(
                Dom::div()
                    .with_class("particle")
                    .with_css_override("top_pos", CssProperty::Top(LayoutTop::px(p.y as f32)))
                    .with_css_override("left_pos", CssProperty::Left(LayoutLeft::px(p.x as f32)))
                    .with_callback(On::MouseDown, Callback(start_drag))
                    .with_callback(On::MouseUp, Callback(end_drag)),
            )
        }
        dom.with_child(
            Button::with_label("Start simulation")
                .dom()
                .with_callback(On::MouseUp, Callback(on_start_sim_btn_clicked)),
        )
        .with_callback(On::MouseOver, Callback(do_drag))
        .with_callback(On::MouseUp, Callback(end_drag))
    }
}

fn do_drag(app_state: &mut AppState<MeshApp>, event: WindowEvent<MeshApp>) -> UpdateScreen {
    app_state.data.modify(|state| {
        if let Some(dragging) = &state.dragging {
            let mut p = dragging.write().unwrap();
            p.x = event.cursor_in_viewport.0 as f64;
            p.y = event.cursor_in_viewport.1 as f64;
        }
    });
    UpdateScreen::Redraw
}

fn start_drag(app_state: &mut AppState<MeshApp>, event: WindowEvent<MeshApp>) -> UpdateScreen {
    app_state.data.modify(|state| {
        state.dragging = event
            .index_path_iter()
            .next()
            .and_then(|idx| state.mesh.particles.get(idx))
            .map(|p| p.clone())
    });
    UpdateScreen::Redraw
}

fn end_drag(app_state: &mut AppState<MeshApp>, _: WindowEvent<MeshApp>) -> UpdateScreen {
    app_state.data.modify(|state| {
        state.dragging = None;
    });
    UpdateScreen::Redraw
}

fn on_start_sim_btn_clicked(
    app_state: &mut AppState<MeshApp>,
    _: WindowEvent<MeshApp>,
) -> UpdateScreen {
    let daemon = Daemon::unique(DaemonCallback(step_daemon));
    app_state.add_daemon(daemon);
    UpdateScreen::Redraw
}

fn step_daemon(
    state: &mut MeshApp,
    _app_resources: &mut AppResources,
) -> (UpdateScreen, TerminateDaemon) {
    println!("running");
    if let Some(dragging) = &state.dragging {
        let (x, y) = {
            let p = dragging.read().unwrap();
            (p.x.clone(), p.y.clone())
        };
        state.mesh.step(0.01);
        let mut p = dragging.write().unwrap();
        p.x = x;
        p.y = y;
    } else {
        state.mesh.step(0.01);
    }
    (UpdateScreen::Redraw, TerminateDaemon::Continue)
}

fn main() {
    //let app = App::new(state::CounterApplication::default(), AppConfig::default());
    let app = App::new(
        MeshApp {
            mesh: mesh::Mesh::new_grid(15.0, 20.0, 20.0, 10, 10),
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
