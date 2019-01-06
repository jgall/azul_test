use azul::prelude::*;

use super::super::widgets::dropdown::*;
use super::mesh::*;
use super::model::*;

pub fn on_start_sim_btn_clicked(
    app_state: &mut AppState<MeshApp>,
    _: WindowEvent<MeshApp>,
) -> UpdateScreen {
    app_state.data.modify(|state| match state {
        MeshApp::Uninitialized(_) => {
            *state = MeshApp::Started(MeshAppState {
                mesh: Mesh::new_grid(15.0, 20.0, 20.0, 20, 20),
                dragging: None,
                dropdown: DropdownState {
                    open: false,
                    selections: vec!["First", "Second", "Third"]
                        .into_iter()
                        .map(str::to_owned)
                        .collect(),
                    selected: None,
                    unselected_label: Some("Select a type".to_owned()),
                },
            })
        }
        MeshApp::Started(_) => (),
    });
    let daemon = Daemon::unique(DaemonCallback(step_daemon));
    app_state.add_daemon(daemon);
    UpdateScreen::Redraw
}

fn step_daemon(
    state: &mut MeshApp,
    _app_resources: &mut AppResources,
) -> (UpdateScreen, TerminateDaemon) {
    match state {
        MeshApp::Uninitialized(_) => (UpdateScreen::DontRedraw, TerminateDaemon::Continue),
        MeshApp::Started(state) => {
            if let Some(dragging) = &state.dragging {
                let (x, y) = {
                    let p = dragging.borrow();
                    (p.x, p.y)
                };
                state.mesh.step(0.01);
                let mut p = dragging.borrow_mut();
                p.x = x;
                p.y = y;
            } else {
                state.mesh.step(0.01);
            }
            (UpdateScreen::Redraw, TerminateDaemon::Continue)
        }
    }
}
