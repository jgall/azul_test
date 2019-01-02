use azul::css::CssProperty;
use azul::css::{LayoutLeft, LayoutTop};
use azul::prelude::*;

use super::super::state::{drag, model::MeshApp};

impl Layout for MeshApp {
    fn layout(&self, _info: WindowInfo<Self>) -> Dom<Self> {
        let mut dom = Dom::new(NodeType::Div);
        match self {
            Self::Started(state) => {
                for p in &state.mesh.particles {
                    let p = p.borrow();
                    dom.add_child(
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
                            .with_callback(On::MouseUp, Callback(drag::end_drag)),
                    )
                }
                dom.with_callback(On::MouseOver, Callback(drag::do_drag))
                    .with_callback(On::MouseUp, Callback(drag::end_drag))
            }
            Self::Uninitialized(u) => dom.with_child(u.dom()),
        }
    }
}
