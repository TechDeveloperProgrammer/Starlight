//! Motor de traducción para Starlight Core
//! 
//! Implementa la lógica de traducción bidireccional entre
//! protocolos Java y Bedrock usando el modelo canónico.

pub mod engine;
pub mod mapper;
pub mod validator;

pub use engine::*;
pub use mapper::*;
pub use validator::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test_translation_module_exists() {
        assert!(true);
    }
}
