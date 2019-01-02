use azul::prelude::*;

use super::model::*;

pub fn do_drag(app_state: &mut AppState<MeshApp>, event: WindowEvent<MeshApp>) -> UpdateScreen {
    app_state.data.modify(|state| match state {
        MeshApp::Started(state) => {
            if let Some(dragging) = &state.dragging {
                let mut p = dragging.borrow_mut();
                p.x = event.cursor_in_viewport.0 as f64;
                p.y = event.cursor_in_viewport.1 as f64;
            }
        }
        MeshApp::Uninitialized => (),
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
                .map(|p| p.clone())
        }
        MeshApp::Uninitialized => (),
    });
    UpdateScreen::Redraw
}

pub fn end_drag(app_state: &mut AppState<MeshApp>, _: WindowEvent<MeshApp>) -> UpdateScreen {
    app_state.data.modify(|state| match state {
        MeshApp::Started(state) => state.dragging = None,
        MeshApp::Uninitialized => (),
    });
    UpdateScreen::Redraw
}
