pub mod entity;
pub mod component;
pub mod archetype;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use archetype::{ArchetypeId, ArchetypeLocation, ArchetypeRef};
use component::Component;
use entity::{EntityId, EntityArchetypeRecord};

use crate::component::ComponentId;

/// Entity Component System
pub struct Ecs {
    /// Map relating entities to their archetypes and their location in the archetype
    pub entity_index: HashMap<EntityId, EntityArchetypeRecord>,
    /// Map relating unique component ids to a set of their owner archetypes and those archetypes' columns
    pub component_index: HashMap<ComponentId, HashMap<ArchetypeId, ArchetypeLocation>>,
    /// Map relating archetypes to the components they are composed of
    pub archetype_index: HashMap<ArchetypeId, Vec<ComponentId>>,
}

impl Ecs {
    /// Gets a component of an entity
    pub fn get_component(&self, entity_id: &EntityId, component_id: &ComponentId)
            -> Option<Rc<RefCell<dyn Component>>> {
        let record = self.entity_index
            .get(entity_id)
            .unwrap();

        let archetype = record.archetype
            .borrow();

        let archetypes = self.component_index
            .get(component_id)
            .unwrap();
 
        if archetypes.contains_key(&archetype.id) {
            let archetype_record = archetypes
                .get(&archetype.id)
                .unwrap();

            archetype.component_insts
                .get(*archetype_record)
                .unwrap()
                .get(record.row)
                .cloned()
        } else {
            None
        }
    }

    pub fn add_component(&mut self, entity: &EntityId, component: &ComponentId)
            -> () {
        let entity_record = self.entity_index
            .get_mut(entity)
            .unwrap();

        let next_archetype: ArchetypeRef = entity_record.archetype
            .borrow_mut()
            .edges
            .get(component)
            .unwrap()
            .add
            .clone();

        todo!();//FIXME move entity to new archetype

        entity_record.row = next_archetype
            .borrow()
            .component_insts
            .get(0)
            .and_then(|row| Some(row.len()))
            .unwrap_or(0);
        entity_record.archetype = next_archetype;
    }
}
