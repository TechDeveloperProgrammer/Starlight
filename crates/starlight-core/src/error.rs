//! Módulo de errores para Starlight Core
//! 
//! Proporciona tipos de error estructurados para manejar fallos
//! en traducción, protocolo, serialización y operaciones internas.

use thiserror::Error;

/// Error principal de Starlight Core
#[derive(Error, Debug)]
pub enum StarlightError {
    #[error("Error de protocolo: {0}")]
    Protocol(String),

    #[error("Error de traducción: {0}")]
    Translation(String),

    #[error("Error de serialización: {0}")]
    Serialization(String),

    #[error("Error de deserialización: {0}")]
    Deserialization(String),

    #[error("Error de versión: {0}")]
    Version(String),

    #[error("Error de entidad: {0}")]
    Entity(String),

    #[error("Error de ítem: {0}")]
    Item(String),

    #[error("Error de bloque: {0}")]
    Block(String),

    #[error("Error de chunk: {0}")]
    Chunk(String),

    #[error("Error de NBT: {0}")]
    Nbt(String),

    #[error("Error de FFI: {0}")]
    Ffi(String),

    #[error("Error interno: {0}")]
    Internal(String),

    #[error("Datos inválidos: {0}")]
    InvalidData(String),

    #[error("Operación no soportada: {0}")]
    Unsupported(String),
}

/// Resultado estándar de Starlight Core
pub type StarlightResult<T> = Result<T, StarlightError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = StarlightError::Protocol("Paquete desconocido".to_string());
        assert!(err.to_string().contains("Protocolo"));
    }

    #[test]
    fn test_result_ok() {
        let result: StarlightResult<i32> = Ok(42);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_err() {
        let result: StarlightResult<i32> = Err(StarlightError::Translation(
            "Mapeo no encontrado".to_string()
        ));
        assert!(result.is_err());
    }
}
