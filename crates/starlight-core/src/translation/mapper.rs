//! Mapeadores de protocolo Java ↔ Bedrock
//! 
//! Define los mapeos entre IDs de paquetes, entidades, ítems y bloques.

use std::collections::HashMap;

/// Mapeo de paquetes Java a Bedrock
pub struct PacketMapper {
    java_to_bedrock: HashMap<u32, u32>,
    bedrock_to_java: HashMap<u32, u32>,
}

impl PacketMapper {
    pub fn new() -> Self {
        let mut mapper = Self {
            java_to_bedrock: HashMap::new(),
            bedrock_to_java: HashMap::new(),
        };
        mapper.init_default_mappings();
        mapper
    }

    fn init_default_mappings(&mut self) {
        // Mapeos placeholder - serán completados con el protocolo real
        self.java_to_bedrock.insert(0x00, 0x01); // KeepAlive
        self.java_to_bedrock.insert(0x01, 0x02); // Login
        self.bedrock_to_java.insert(0x01, 0x00);
        self.bedrock_to_java.insert(0x02, 0x01);
    }

    pub fn map_java_to_bedrock(&self, java_id: u32) -> Option<u32> {
        self.java_to_bedrock.get(&java_id).copied()
    }

    pub fn map_bedrock_to_java(&self, bedrock_id: u32) -> Option<u32> {
        self.bedrock_to_java.get(&bedrock_id).copied()
    }

    pub fn add_mapping(&mut self, java_id: u32, bedrock_id: u32) {
        self.java_to_bedrock.insert(java_id, bedrock_id);
        self.bedrock_to_java.insert(bedrock_id, java_id);
    }
}

impl Default for PacketMapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_mapper_creation() {
        let mapper = PacketMapper::new();
        assert!(!mapper.java_to_bedrock.is_empty());
    }

    #[test]
    fn test_java_to_bedrock_mapping() {
        let mapper = PacketMapper::new();
        let result = mapper.map_java_to_bedrock(0x00);
        assert_eq!(result, Some(0x01));
    }

    #[test]
    fn test_bedrock_to_java_mapping() {
        let mapper = PacketMapper::new();
        let result = mapper.map_bedrock_to_java(0x01);
        assert_eq!(result, Some(0x00));
    }

    #[test]
    fn test_add_custom_mapping() {
        let mut mapper = PacketMapper::new();
        mapper.add_mapping(0xFF, 0xFE);
        assert_eq!(mapper.map_java_to_bedrock(0xFF), Some(0xFE));
        assert_eq!(mapper.map_bedrock_to_java(0xFE), Some(0xFF));
    }

    #[test]
    fn test_unknown_mapping() {
        let mapper = PacketMapper::new();
        let result = mapper.map_java_to_bedrock(0xFFFF);
        assert_eq!(result, None);
    }
}
