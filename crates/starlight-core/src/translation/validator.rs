//! Validador de datos traducidos
//! 
//! Verifica que los datos traducidos sean válidos y consistentes.

use crate::error::{StarlightError, StarlightResult};
use crate::model::canonical::*;

/// Validador de traducciones
pub struct TranslationValidator {
    strict_mode: bool,
}

impl TranslationValidator {
    pub fn new(strict_mode: bool) -> Self {
        Self { strict_mode }
    }

    /// Valida una entidad después de la traducción
    pub fn validate_entity(&self, entity: &Entity) -> StarlightResult<()> {
        if self.strict_mode {
            if entity.position.x.abs() > 30_000_000.0
                || entity.position.y.abs() > 30_000_000.0
                || entity.position.z.abs() > 30_000_000.0
            {
                return Err(StarlightError::Entity(
                    "Posición fuera de límites".to_string()
                ));
            }
        }

        if entity.health < 0.0 {
            return Err(StarlightError::Entity(
                "Salud negativa no permitida".to_string()
            ));
        }

        Ok(())
    }

    /// Valida un ítem después de la traducción
    pub fn validate_item(&self, item: &Item) -> StarlightResult<()> {
        if item.id.is_empty() {
            return Err(StarlightError::Item(
                "ID de ítem vacío".to_string()
            ));
        }

        if item.count == 0 && item.damage != 0 {
            return Err(StarlightError::Item(
                "Ítem con count 0 pero con damage".to_string()
            ));
        }

        Ok(())
    }

    /// Valida un bloque después de la traducción
    pub fn validate_block(&self, block: &Block) -> StarlightResult<()> {
        if block.id.is_empty() {
            return Err(StarlightError::Block(
                "ID de bloque vacío".to_string()
            ));
        }

        Ok(())
    }

    /// Valida datos NBT
    pub fn validate_nbt(&self, nbt: &NbtValue) -> StarlightResult<()> {
        match nbt {
            NbtValue::Compound(pairs) => {
                // Verificar duplicados en compound
                let mut seen = std::collections::HashSet::new();
                for (key, _) in pairs {
                    if !seen.insert(key.clone()) {
                        return Err(StarlightError::Nbt(
                            format!("Clave duplicada en NBT: {}", key)
                        ));
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Valida un paquete completo
    pub fn validate_packet(&self, packet: &[u8]) -> StarlightResult<()> {
        if packet.is_empty() {
            return Err(StarlightError::InvalidData(
                "Paquete vacío".to_string()
            ));
        }

        if self.strict_mode && packet.len() > 1_000_000 {
            return Err(StarlightError::InvalidData(
                "Paquete demasiado grande".to_string()
            ));
        }

        Ok(())
    }
}

impl Default for TranslationValidator {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = TranslationValidator::new(false);
        assert!(!validator.strict_mode);
    }

    #[test]
    fn test_validate_entity_ok() {
        let validator = TranslationValidator::new(true);
        let entity = Entity::new(
            EntityId(1),
            EntityType::Zombie,
            Position::new(100.0, 64.0, -50.0),
        );
        assert!(validator.validate_entity(&entity).is_ok());
    }

    #[test]
    fn test_validate_entity_out_of_bounds() {
        let validator = TranslationValidator::new(true);
        let entity = Entity::new(
            EntityId(1),
            EntityType::Zombie,
            Position::new(50_000_000.0, 0.0, 0.0),
        );
        assert!(validator.validate_entity(&entity).is_err());
    }

    #[test]
    fn test_validate_item_ok() {
        let validator = TranslationValidator::new(true);
        let item = Item {
            id: "minecraft:diamond_sword".to_string(),
            count: 1,
            damage: 0,
            nbt: None,
        };
        assert!(validator.validate_item(&item).is_ok());
    }

    #[test]
    fn test_validate_item_empty_id() {
        let validator = TranslationValidator::new(true);
        let item = Item {
            id: "".to_string(),
            count: 1,
            damage: 0,
            nbt: None,
        };
        assert!(validator.validate_item(&item).is_err());
    }

    #[test]
    fn test_validate_block_ok() {
        let validator = TranslationValidator::new(true);
        let block = Block {
            id: "minecraft:stone".to_string(),
            states: vec![],
            nbt: None,
        };
        assert!(validator.validate_block(&block).is_ok());
    }

    #[test]
    fn test_validate_packet_empty() {
        let validator = TranslationValidator::new(true);
        assert!(validator.validate_packet(&[]).is_err());
    }

    #[test]
    fn test_validate_packet_ok() {
        let validator = TranslationValidator::new(true);
        assert!(validator.validate_packet(&[0x01, 0x02]).is_ok());
    }
}
