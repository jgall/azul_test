use std::sync::{Arc, RwLock};

use super::mesh::*;



pub struct MeshApp {
    pub mesh: Mesh,
    pub dragging: Option<Arc<RwLock<Particle>>>,
}
