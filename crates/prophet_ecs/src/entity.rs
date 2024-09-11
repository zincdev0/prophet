use crate::archetype::ArchetypeRef;

/// A unique identifier for each different entity
/// 
/// Bit structure:
/// 32 index;
/// 16 generation;
/// 8 flags;
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct EntityId(pub u64);

impl EntityId {
    pub fn new(index: u32, gener: u16, flags: u8)
            -> Self {
        Self((index as u64)
            + (gener as u64) << 32
            + (flags as u64) << 48)
    }

    pub fn get_index(&self) -> u32 {
        self.0 as u32
    }

    pub fn get_gener(&self) -> u16 {
        (self.0 >> 32) as u16
    }

    pub fn get_flags(&self) -> u8 {
        (self.0 >> 48) as u8
    }
}

/// Points to an entity's archetype and row in archetype
#[derive(Clone)]
pub struct EntityArchetypeRecord {
    pub archetype: ArchetypeRef,
    pub row: usize,
}
