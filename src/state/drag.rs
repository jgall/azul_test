use azul::css::CssProperty;
use azul::css::{LayoutLeft, LayoutTop};
use azul::{prelude::*, widgets::button::Button};

use super::model::*;

pub fn do_drag(app_state: &mut AppState<MeshApp>, event: WindowEvent<MeshApp>) -> UpdateScreen {
    app_state.data.modify(|state| {
        if let Some(dragging) = &state.dragging {
            let mut p = dragging.write().unwrap();
            p.x = event.cursor_in_viewport.0 as f64;
            p.y = event.cursor_in_viewport.1 as f64;
        }
    });
    UpdateScreen::Redraw
}

pub fn start_drag(app_state: &mut AppState<MeshApp>, event: WindowEvent<MeshApp>) -> UpdateScreen {
    app_state.data.modify(|state| {
        state.dragging = event
            .index_path_iter()
            .next()
            .and_then(|idx| state.mesh.particles.get(idx))
            .map(|p| p.clone())
    });
    UpdateScreen::Redraw
}

pub fn end_drag(app_state: &mut AppState<MeshApp>, _: WindowEvent<MeshApp>) -> UpdateScreen {
    app_state.data.modify(|state| {
        state.dragging = None;
    });
    UpdateScreen::Redraw
}
