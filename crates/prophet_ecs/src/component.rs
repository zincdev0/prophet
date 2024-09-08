use std::{cell::RefCell, rc::Rc};

/// A unique identifier for each different component type
#[derive(PartialEq, Eq, Hash)]
pub struct ComponentId(pub u64);

/// Instances of structs dyn Component will be created containing component data
/// Contained within Box<dyn Component> (heap allocated)
pub trait Component {}

pub type ComponentRef = Rc<RefCell<dyn Component>>;
