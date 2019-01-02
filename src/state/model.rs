use std::cell::RefCell;
use std::rc::Rc;

use super::mesh::*;

pub enum MeshApp {
    Started(MeshAppState),
    Uninitialized(Uninitialized),
}

pub struct Uninitialized {}

pub struct MeshAppState {
    pub mesh: Mesh,
    pub dragging: Option<Rc<RefCell<Particle>>>,
}
