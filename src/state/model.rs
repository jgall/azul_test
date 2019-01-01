use std::sync::{Arc, RwLock};

use super::mesh::*;

pub enum MeshApp {
    Started(MeshAppState),
    Uninitialized,
}

pub struct MeshAppState {
    pub mesh: Mesh,
    pub dragging: Option<Arc<RwLock<Particle>>>,
}
