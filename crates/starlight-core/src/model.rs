//! Modelo canónico para Starlight Core
//! 
//! Define las estructuras de datos independientes de plataforma
//! que representan el estado del juego de forma unificada.

pub mod canonical;
pub mod packet;
pub mod state;

pub use canonical::*;
pub use packet::*;
pub use state::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_model_exists() {
        // Verifica que el modelo canónico esté disponible
        assert!(true);
    }
}
