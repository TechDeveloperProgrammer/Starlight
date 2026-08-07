//! Motor de traducción principal
//! 
//! Coordina la traducción de paquetes entre Java y Bedrock
//! usando el modelo canónico como intermediario.

use crate::error::{StarlightError, StarlightResult};
use crate::model::canonical::*;
use crate::model::packet::TranslatedPacket;
use tracing::{debug, info, warn};

/// Configuración del motor de traducción
#[derive(Debug, Clone)]
pub struct TranslationConfig {
    pub java_version: String,
    pub bedrock_version: String,
    pub strict_mode: bool,
    pub log_translations: bool,
}

impl Default for TranslationConfig {
    fn default() -> Self {
        Self {
            java_version: "1.20.4".to_string(),
            bedrock_version: "1.20.50".to_string(),
            strict_mode: false,
            log_translations: true,
        }
    }
}

/// Motor de traducción bidireccional
pub struct TranslationEngine {
    config: TranslationConfig,
    packets_translated: u64,
    packets_failed: u64,
}

impl TranslationEngine {
    /// Crea un nuevo motor de traducción
    pub fn new(config: TranslationConfig) -> Self {
        if config.log_translations {
            info!(
                "TranslationEngine initialized: Java {} ↔ Bedrock {}",
                config.java_version, config.bedrock_version
            );
        }
        Self {
            config,
            packets_translated: 0,
            packets_failed: 0,
        }
    }

    /// Traduce un paquete Java a Bedrock
    pub fn java_to_bedrock(&mut self, packet: &Packet) -> StarlightResult<TranslatedPacket> {
        debug!("Translating Java packet {} -> Bedrock", packet.id);
        
        // Placeholder: implementación real requiere mapeo de protocolos
        let translated = TranslatedPacket::new(
            packet.id,
            packet.id, // ID temporal, será mapeado correctamente
            packet.data.clone(),
        );

        self.packets_translated += 1;
        Ok(translated)
    }

    /// Traduce un paquete Bedrock a Java
    pub fn bedrock_to_java(&mut self, packet: &Packet) -> StarlightResult<TranslatedPacket> {
        debug!("Translating Bedrock packet {} -> Java", packet.id);
        
        // Placeholder: implementación real requiere mapeo de protocolos
        let translated = TranslatedPacket::new(
            packet.id,
            packet.id, // ID temporal, será mapeado correctamente
            packet.data.clone(),
        );

        self.packets_translated += 1;
        Ok(translated)
    }

    /// Obtiene estadísticas de traducción
    pub fn get_stats(&self) -> TranslationStats {
        TranslationStats {
            packets_translated: self.packets_translated,
            packets_failed: self.packets_failed,
            success_rate: if self.packets_translated + self.packets_failed > 0 {
                self.packets_translated as f64 
                    / (self.packets_translated + self.packets_failed) as f64
            } else {
                1.0
            },
        }
    }

    /// Valida datos antes de la traducción
    pub fn validate(&self, data: &[u8]) -> StarlightResult<()> {
        if data.is_empty() {
            return Err(StarlightError::InvalidData(
                "Datos vacíos no permitidos".to_string()
            ));
        }
        Ok(())
    }
}

/// Estadísticas de traducción
#[derive(Debug, Clone)]
pub struct TranslationStats {
    pub packets_translated: u64,
    pub packets_failed: u64,
    pub success_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let config = TranslationConfig::default();
        let engine = TranslationEngine::new(config);
        assert_eq!(engine.packets_translated, 0);
    }

    #[test]
    fn test_java_to_bedrock_translation() {
        let mut engine = TranslationEngine::new(TranslationConfig::default());
        let packet = Packet {
            id: 1,
            name: "TestPacket".to_string(),
            data: vec![0x01, 0x02, 0x03],
            timestamp: 0,
        };
        let result = engine.java_to_bedrock(&packet);
        assert!(result.is_ok());
        let translated = result.unwrap();
        assert_eq!(translated.original_id, 1);
    }

    #[test]
    fn test_bedrock_to_java_translation() {
        let mut engine = TranslationEngine::new(TranslationConfig::default());
        let packet = Packet {
            id: 2,
            name: "TestPacket".to_string(),
            data: vec![0x04, 0x05],
            timestamp: 0,
        };
        let result = engine.bedrock_to_java(&packet);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_empty_data() {
        let engine = TranslationEngine::new(TranslationConfig::default());
        let result = engine.validate(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_non_empty_data() {
        let engine = TranslationEngine::new(TranslationConfig::default());
        let result = engine.validate(&[0x01, 0x02]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_stats() {
        let mut engine = TranslationEngine::new(TranslationConfig::default());
        let packet = Packet {
            id: 1,
            name: "Test".to_string(),
            data: vec![],
            timestamp: 0,
        };
        let _ = engine.java_to_bedrock(&packet);
        let stats = engine.get_stats();
        assert_eq!(stats.packets_translated, 1);
        assert_eq!(stats.packets_failed, 0);
    }
}
