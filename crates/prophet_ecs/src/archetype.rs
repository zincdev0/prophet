use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::component::{ComponentId, ComponentRef};


/// An identifier of an Archetype
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ArchetypeId(
    pub(crate) u64,
);


/// A type of an entity, containing a set of components.
pub struct Archetype {
    /// A unique identifier
    pub(crate) id: ArchetypeId,
    
    /// The component ids associated with the Archetype
    pub(crate) component_ids: Vec<ComponentId>,
    
    /// The list (indexed by component id) of component instances for anything sharing the Archetype
    pub(crate) component_insts: Vec<Vec<ComponentRef>>,
    
    /// Cached references to Archetypes resulting if a component is added or removed
    pub(crate) edges: HashMap<ComponentId, ArchetypeEdge>,
}

impl Archetype {
    pub(crate) fn get_add(&self, component: &ComponentId)
            -> &ArchetypeRef {
        &self.edges
            [component]
            .add
    }
    
    pub(crate) fn get_rem(&self, component: &ComponentId)
            -> &ArchetypeRef {
        &self.edges
            [component]
            .rem
    }
}


/// Reference/handle to an Archetype
pub type ArchetypeRef = Rc<RefCell<Archetype>>;


/// A set of references to archetypes which would result if a component is added or removed
pub struct ArchetypeEdge {
    pub add: ArchetypeRef,
    pub rem: ArchetypeRef,
}


// Points to an archetypes' column
pub type ArchetypeColumn = usize;
