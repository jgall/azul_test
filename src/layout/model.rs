use azul::css::CssProperty;
use azul::css::{LayoutLeft, LayoutTop};
use azul::prelude::*;

use super::super::state::{drag, model::MeshApp};
use super::super::widgets::dropdown::*;

impl Layout for MeshApp {
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        match self {
            Self::Started(state) => Dom::div()
                .with_child(
                    Dropdown::new()
                        .bind(info.window, &state.dropdown, &self)
                        .dom(&state.dropdown),
                )
                .with_child(
                    state
                        .mesh
                        .particles
                        .iter()
                        .map(|p| p.borrow())
                        .map(|p| {
                            Dom::div()
                                .with_class("particle")
                                .with_css_override(
                                    "top_pos",
                                    CssProperty::Top(LayoutTop::px(p.y as f32)),
                                )
                                .with_css_override(
                                    "left_pos",
                                    CssProperty::Left(LayoutLeft::px(p.x as f32)),
                                )
                                .with_callback(On::MouseDown, Callback(drag::start_drag))
                                .with_callback(On::MouseUp, Callback(drag::end_drag))
                        })
                        .fold(Dom::new(NodeType::Div), |dom, p| dom.with_child(p))
                        .with_callback(On::MouseOver, Callback(drag::do_drag))
                        .with_callback(On::MouseUp, Callback(drag::end_drag)),
                ),
            Self::Uninitialized(u) => u.dom(),
        }
    }
}
