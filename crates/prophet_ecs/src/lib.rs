pub mod entity;
pub mod component;
pub mod archetype;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use archetype::{Archetype, ArchetypeEdge, ArchetypeId, ArchetypeColumn, ArchetypeRef};
use component::{Component, ComponentId, ComponentRef};
use entity::{EntityId, EntityRecord};


/// Entity Component System
pub struct Ecs {
    /// Entities to their archetypes
    pub entity_index: HashMap<EntityId, EntityRecord>,

    /// Components to where they are in archetypes
    pub component_index: HashMap<ComponentId, HashMap<ArchetypeId, ArchetypeColumn>>,

    /// ArchetypeIds to their Archetypes
    pub archetype_index: HashMap<ArchetypeId, ArchetypeRef>,
}

impl Ecs {
    pub fn new()
            -> Ecs {
        Ecs {
            entity_index: HashMap::<EntityId, EntityRecord>::new(),
            component_index: HashMap::<ComponentId, HashMap<ArchetypeId, ArchetypeColumn>>::new(),
            archetype_index: HashMap::<ArchetypeId, ArchetypeRef>::new(),
        }
    }

    /// Creates a new component type
    pub fn new_component(&mut self)
            -> ComponentId {
        let comp_id = self
            .component_index
            .iter()
            .last()
            .and_then(|(comp_id, _archs)|
                Some(ComponentId(comp_id.0 + 1)))
            .unwrap_or(ComponentId(0));
        let mut next_arch_idx = self
            .archetype_index
            .iter()
            .last()
            .and_then(|(arch_id, _arch_ref)|
                Some(arch_id.0 + 1))
            .unwrap_or(0);

        let mut arch_locs = HashMap::<ArchetypeId, ArchetypeColumn>::new();
        let mut arch_map = HashMap::<ArchetypeId, ArchetypeRef>::new();
        self.archetype_index
            .iter_mut()
            .for_each(|(_arch_id, old_arch_ref)| {
                let arch_id = ArchetypeId(next_arch_idx);
                let new_arch_ref = Rc::new(RefCell::new(Archetype {
                    id: arch_id,
                    component_ids: {
                        let mut tmp = old_arch_ref
                            .borrow()
                            .component_ids
                            .clone();
                        arch_locs.insert(arch_id, tmp.len());
                        tmp.push(comp_id);
                        tmp
                    },
                    component_insts: old_arch_ref
                        .borrow()
                        .component_insts
                        .iter()
                        .map(|_col| Vec::<ComponentRef>::new())
                        .collect(),
                    edges: HashMap::<ComponentId, ArchetypeEdge>::new(),
                }));
                next_arch_idx += 1;

                new_arch_ref.borrow_mut().edges = old_arch_ref
                    .borrow()
                    .edges
                    .iter()
                    .map(|(comp_id, _arch_edge)| {
                        (*comp_id, ArchetypeEdge {
                            add: new_arch_ref.clone(),
                            rem: old_arch_ref.clone(),
                        })
                    })
                    .collect();

                old_arch_ref
                    .borrow_mut()
                    .edges
                    .insert(comp_id, ArchetypeEdge {
                        add: new_arch_ref.clone(),
                        rem: old_arch_ref.clone(),
                    });

                arch_map.insert(arch_id, new_arch_ref);
            });

        self.component_index
            .insert(comp_id, arch_locs);
        
        comp_id
    }

    pub fn get_record(&self, ent_id: &EntityId)
            -> &EntityRecord {
        self.entity_index
            .get(ent_id)
            .unwrap()
    }

    pub fn get_record_mut(&mut self, ent_id: &EntityId)
            -> &mut EntityRecord {
        self.entity_index
            .get_mut(ent_id)
            .unwrap()
    }

    pub fn replace_record(&mut self, ent_id: EntityId, rec: EntityRecord)
            -> &EntityRecord {
        self.entity_index
            .insert(ent_id, rec);
        self.get_record(&ent_id)
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
    pub fn add_component(&mut self, ent_id: &EntityId, comp_id: &ComponentId, comp_inst: ComponentRef)
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

        // Save copy of old record
        let prev_rec = self
            .get_record(ent_id)
            .clone();

        // Adjust record
        let next_rec = self.replace_record(ent_id.clone(), EntityRecord {
            archetype: prev_rec.archetype
                .borrow()
                .get_add(comp_id)
                .clone(),
            row: prev_rec.archetype
                .borrow()
                .get_add(comp_id)
                .borrow()
                .component_insts
                .get(0)
                .and_then(|row| Some(row.len()))
                .unwrap_or(0),
        }).clone();
        let next_arch = next_rec.archetype.clone();

        prev_rec.archetype
            .borrow_mut()
            .component_insts
            .iter_mut()
            .enumerate()
            .for_each(|(col_idx, rows)| {
                next_arch
                    .borrow_mut()
                    .component_insts
                    .get_mut(self.component_index
                        [next_arch
                            .borrow_mut()
                            .component_ids
                            .get(col_idx)
                            .unwrap()]
                        [&next_rec.archetype.borrow().id])
                    .unwrap()
                    .push(rows.remove(prev_rec.row));
            });

        let what = next_arch
            .borrow()
            .component_insts
            .get_mut(comp_id)
            .unwrap();

        // Decrement row count for entities after removed components
        self.entity_index
            .iter_mut()
            .filter(|(_ent, rec)| rec.archetype.borrow().id == prev_rec.archetype.borrow().id)
            .filter(|(_ent, rec)| rec.row > prev_rec.row)
            .for_each(|(_ent, rec)| rec.row -= 1);
    }
}
