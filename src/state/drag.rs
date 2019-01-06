use azul::prelude::*;

use super::model::*;

pub fn do_drag(app_state: &mut AppState<MeshApp>, event: WindowEvent<MeshApp>) -> UpdateScreen {
    app_state.data.modify(|state| match state {
        MeshApp::Started(state) => {
            if let Some(dragging) = &state.dragging {
                let mut p = dragging.borrow_mut();
                p.x = f64::from(event.cursor_in_viewport.0);
                p.y = f64::from(event.cursor_in_viewport.1);
            }
        }
        MeshApp::Uninitialized(_) => (),
    });
    UpdateScreen::Redraw
}

pub fn start_drag(app_state: &mut AppState<MeshApp>, event: WindowEvent<MeshApp>) -> UpdateScreen {
    app_state.data.modify(|state| match state {
        MeshApp::Started(state) => {
            state.dragging = event
                .index_path_iter()
                .next()
                .and_then(|idx| state.mesh.particles.get(idx))
                .cloned()
        }
        MeshApp::Uninitialized(_) => (),
    });
    UpdateScreen::Redraw
}

pub fn end_drag(app_state: &mut AppState<MeshApp>, _: WindowEvent<MeshApp>) -> UpdateScreen {
    app_state.data.modify(|state| match state {
        MeshApp::Started(state) => state.dragging = None,
        MeshApp::Uninitialized(_) => (),
    });
    UpdateScreen::Redraw
}
