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
    /// Map relating unique component ids to a set of their owner archetypes and those archetypes'
    /// columns
    pub component_index: HashMap<ComponentId, HashMap<ArchetypeId, ArchetypeLocation>>,
    /// Map relating archetypes to the components they are composed of
    pub archetype_index: HashMap<ArchetypeId, Vec<ComponentId>>,
}

impl Ecs {
    pub fn new()
            -> Ecs {
        Ecs {
            entity_index: HashMap::<EntityId, EntityArchetypeRecord>::new(),
            component_index: HashMap::<ComponentId, HashMap<ArchetypeId, ArchetypeLocation>>::new(),
            archetype_index: HashMap::<ArchetypeId, Vec<ComponentId>>::new(),
        }
    }

    pub fn get_record(&self, ent_id: &EntityId)
            -> &EntityArchetypeRecord {
        self.entity_index
            .get(ent_id)
            .unwrap()
    }

    pub fn get_record_mut(&mut self, ent_id: &EntityId)
            -> &mut EntityArchetypeRecord {
        self.entity_index
            .get_mut(ent_id)
            .unwrap()
    }

    pub fn replace_record(&mut self, ent_id: EntityId, rec: EntityArchetypeRecord)
            -> () {
        self.entity_index
            .insert(ent_id, rec);
    }

    /// Gets a component of an entity
    /// 
    /// Time complexity: O(1)
    pub fn get_component(&self, ent_id: &EntityId, comp_id: &ComponentId)
            -> Option<Rc<RefCell<dyn Component>>> {
        let ent_rec = self.get_record(ent_id);

        self.component_index
            .get(comp_id)
            .unwrap()
            .get(&ent_rec.archetype.borrow().id)
            .and_then(|arch_col| ent_rec
                .archetype
                .borrow()
                .component_insts
                .get(*arch_col)
                .unwrap()
                .get(ent_rec.row)
                .cloned())
    }

    /// Time complexity: O(n in entity_index)
    pub fn add_component(&mut self, ent_id: &EntityId, comp_id: &ComponentId)
            -> () {
        // problem:
        // i need to move the entity to its new archetype.
        // this leaves the slot in its previous component vectors empty.
        // options to fix it:
        //     1. make it a vector of Options, leave None in its place
        //     2. take it out and leave an uninitialized value
        //     3. remove it, find other entities with their row over that
        //        value and decrement their row
        // cons 1, 2: will eventually run out of components
        // cons 3: very slow both to folding remove and also to decrement
        // choice: 3 for now
        // reasoning: 1 and 2 are a memory leak
        // this comment will be left in case it needs to be changed
        let prev_rec = self
            .get_record(ent_id)
            .clone();

        let mut next_rec = EntityArchetypeRecord {
            archetype: prev_rec.archetype
                .borrow()
                .add(comp_id)
                .clone(),
            row: 0,
        };

        next_rec.row = prev_rec.archetype
            .borrow()
            .component_insts
            .get(0)
            .and_then(|row| Some(row.len()))
            .unwrap_or(0);

        self.replace_record(ent_id.clone(), next_rec);

        // Decrement row count for entities after removed components
        self.entity_index
            .iter_mut()
            .filter(|(_ent, rec)| rec.archetype.borrow().id == prev_rec.archetype.borrow().id)
            .filter(|(_ent, rec)| rec.row > prev_rec.row)
            .for_each(|(_ent, rec)| rec.row -= 1);

        // Remove components from previous archetype
        todo!();
    }
}
