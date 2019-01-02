use super::super::state::{model, start};
use azul::dom::Dom;
use azul::{prelude::*, widgets::button::Button};

impl model::Uninitialized {
    pub fn dom(&self) -> Dom<model::MeshApp> {
        Button::with_label("Start simulation")
            .dom()
            .with_callback(On::MouseUp, Callback(start::on_start_sim_btn_clicked))
    }
}
