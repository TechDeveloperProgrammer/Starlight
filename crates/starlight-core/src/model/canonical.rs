//! Modelo de datos canónico independiente de plataforma
//! 
//! Estas estructuras representan el estado del juego de forma
//! neutral, permitiendo traducción bidireccional Java ↔ Bedrock.

use serde::{Deserialize, Serialize};

/// Identificador único de entidad en el modelo canónico
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub u32);

/// Identificador único de jugador
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub u32);

/// Posición en el mundo
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// Rotación (yaw, pitch)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Rotation {
    pub fn new(yaw: f32, pitch: f32) -> Self {
        Self { yaw, pitch }
    }
}

/// Tipo de entidad canónica
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    Player,
    Zombie,
    Skeleton,
    Creeper,
    Spider,
    Pig,
    Cow,
    Sheep,
    Chicken,
    Item,
    FallingBlock,
    Projectile,
    Other(String),
}

/// Entidad en el modelo canónico
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub entity_type: EntityType,
    pub position: Position,
    pub rotation: Rotation,
    pub velocity: (f32, f32, f32),
    pub health: f32,
    pub metadata: Vec<(String, NbtValue)>,
}

impl Entity {
    pub fn new(id: EntityId, entity_type: EntityType, position: Position) -> Self {
        Self {
            id,
            entity_type,
            position,
            rotation: Rotation::new(0.0, 0.0),
            velocity: (0.0, 0.0, 0.0),
            health: 20.0,
            metadata: Vec::new(),
        }
    }
}

/// Valor NBT canónico
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NbtValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<NbtValue>),
    Compound(Vec<(String, NbtValue)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

/// Ítem en el modelo canónico
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub count: u8,
    pub damage: i16,
    pub nbt: Option<NbtValue>,
}

/// Bloque en el modelo canónico
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: String,
    pub states: Vec<(String, String)>,
    pub nbt: Option<NbtValue>,
}

/// Paquete canónico
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    pub id: u32,
    pub name: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(100.0, 64.0, -50.0);
        assert_eq!(pos.x, 100.0);
        assert_eq!(pos.y, 64.0);
        assert_eq!(pos.z, -50.0);
    }

    #[test]
    fn test_entity_creation() {
        let entity = Entity::new(
            EntityId(1),
            EntityType::Zombie,
            Position::new(0.0, 0.0, 0.0),
        );
        assert_eq!(entity.id, EntityId(1));
        assert_eq!(entity.health, 20.0);
    }

    #[test]
    fn test_nbt_value() {
        let nbt = NbtValue::String("test".to_string());
        match nbt {
            NbtValue::String(s) => assert_eq!(s, "test"),
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn test_rotation() {
        let rot = Rotation::new(90.0, -45.0);
        assert_eq!(rot.yaw, 90.0);
        assert_eq!(rot.pitch, -45.0);
    }
}
