use std::{cell::RefCell, rc::Rc};


/// A unique identifier for each different component type
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ComponentId(
    pub(crate) u64,
);


/// Component data and functionality
pub trait Component {}


/// Reference/handle to a Component
pub type ComponentRef = Rc<RefCell<dyn Component>>;
