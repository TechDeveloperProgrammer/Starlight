//! Modelo packet para Starlight Core
//! 
//! Define estructuras para paquetes canónicos.

use crate::model::canonical::Packet;

/// Dirección del paquete
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketDirection {
    ClientBound,
    ServerBound,
}

/// Estado del paquete después de la traducción
#[derive(Debug, Clone)]
pub struct TranslatedPacket {
    pub original_id: u32,
    pub translated_id: u32,
    pub data: Vec<u8>,
    pub success: bool,
    pub warnings: Vec<String>,
}

impl TranslatedPacket {
    pub fn new(original_id: u32, translated_id: u32, data: Vec<u8>) -> Self {
        Self {
            original_id,
            translated_id,
            data,
            success: true,
            warnings: Vec::new(),
        }
    }

    pub fn with_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }

    pub fn failed(mut self) -> Self {
        self.success = false;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translated_packet_creation() {
        let packet = TranslatedPacket::new(1, 2, vec![0x01, 0x02]);
        assert_eq!(packet.original_id, 1);
        assert_eq!(packet.translated_id, 2);
        assert!(packet.success);
    }

    #[test]
    fn test_translated_packet_with_warning() {
        let packet = TranslatedPacket::new(1, 2, vec![])
            .with_warning("Datos parciales".to_string());
        assert_eq!(packet.warnings.len(), 1);
    }

    #[test]
    fn test_translated_packet_failed() {
        let packet = TranslatedPacket::new(1, 2, vec![]).failed();
        assert!(!packet.success);
    }
}
