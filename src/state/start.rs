use azul::css::CssProperty;
use azul::css::{LayoutLeft, LayoutTop};
use azul::{prelude::*, widgets::button::Button};

use super::model::*;

pub fn on_start_sim_btn_clicked(
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
