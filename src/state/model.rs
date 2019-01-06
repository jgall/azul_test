use super::super::widgets::dropdown::*;
use super::mesh::*;
use std::cell::RefCell;
use std::rc::Rc;

pub enum MeshApp {
    Started(MeshAppState),
    Uninitialized(Uninitialized),
}

pub struct Uninitialized {}

pub struct MeshAppState {
    pub mesh: Mesh,
    pub dragging: Option<Rc<RefCell<Particle>>>,
    pub dropdown: DropdownState,
}
