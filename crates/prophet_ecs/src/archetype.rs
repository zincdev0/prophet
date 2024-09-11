use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::component::{Component, ComponentId};

/// An identifier of an Archetype
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ArchetypeId(pub u64);

/// Reference/handle to an Archetype
pub type ArchetypeRef = Rc<RefCell<Archetype>>;

/// A type of an entity, containing a set of components.
pub struct Archetype {
    /// A unique identifier
    pub id: ArchetypeId,
    /// The component ids associated with the Archetype
    pub component_ids: Vec<ComponentId>,
    /// The list (indexed by component id) of component instances for anything sharing the Archetype
    pub component_insts: Vec<Vec<Rc<RefCell<dyn Component>>>>,
    /// Cached references to Archetypes resulting if a component is added or removed
    pub edges: HashMap<ComponentId, ArchetypeEdge>,
}

impl Archetype {
    pub fn add(&self, component: &ComponentId)
            -> &ArchetypeRef {
        &self.edges
            [component]
            .add
    }

    pub fn rem(&self, component: &ComponentId)
            -> &ArchetypeRef {
        &self.edges
            [component]
            .rem
    }
}

/// A set of references to archetypes which would result if a component is added or removed
pub struct ArchetypeEdge {
    pub add: ArchetypeRef,
    pub rem: ArchetypeRef,
}

// Points to an archetypes' column
pub type ArchetypeLocation = usize;
