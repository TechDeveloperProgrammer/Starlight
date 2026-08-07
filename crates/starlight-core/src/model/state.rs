//! Modelo de estado para Starlight Core
//! 
//! Define el estado del mundo y entidades.

use crate::model::canonical::{Entity, EntityId, Position, Block, Item};
use std::collections::HashMap;

/// Estado del mundo canónico
#[derive(Debug, Clone)]
pub struct WorldState {
    pub entities: HashMap<EntityId, Entity>,
    pub blocks: HashMap<(i32, i32, i32), Block>,
    pub time: u64,
    pub difficulty: u8,
    pub game_mode: u8,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            blocks: HashMap::new(),
            time: 0,
            difficulty: 1,
            game_mode: 0,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id.clone(), entity);
    }

    pub fn remove_entity(&mut self, id: &EntityId) -> Option<Entity> {
        self.entities.remove(id)
    }

    pub fn get_entity(&self, id: &EntityId) -> Option<&Entity> {
        self.entities.get(id)
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: Block) {
        self.blocks.insert((x, y, z), block);
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<&Block> {
        self.blocks.get(&(x, y, z))
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::canonical::{EntityId, EntityType, Position, Entity};

    #[test]
    fn test_world_state_creation() {
        let state = WorldState::new();
        assert!(state.entities.is_empty());
        assert!(state.blocks.is_empty());
    }

    #[test]
    fn test_add_entity() {
        let mut state = WorldState::new();
        let entity = Entity::new(
            EntityId(1),
            EntityType::Zombie,
            Position::new(0.0, 0.0, 0.0),
        );
        state.add_entity(entity);
        assert_eq!(state.entities.len(), 1);
    }

    #[test]
    fn test_remove_entity() {
        let mut state = WorldState::new();
        let entity = Entity::new(
            EntityId(1),
            EntityType::Zombie,
            Position::new(0.0, 0.0, 0.0),
        );
        state.add_entity(entity);
        let removed = state.remove_entity(&EntityId(1));
        assert!(removed.is_some());
        assert!(state.entities.is_empty());
    }

    #[test]
    fn test_set_block() {
        let mut state = WorldState::new();
        let block = Block {
            id: "minecraft:stone".to_string(),
            states: vec![],
            nbt: None,
        };
        state.set_block(10, 64, -5, block);
        assert!(state.get_block(10, 64, -5).is_some());
    }
}
