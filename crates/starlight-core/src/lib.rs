//! Starlight Core - Núcleo de traducción bidireccional Java ↔ Bedrock
//! 
//! Este crate proporciona el modelo canónico y motor de traducción
//! para convertir protocolos, entidades, ítems y estados entre
//! Minecraft Java Edition y Bedrock Edition.

pub mod error;
pub mod model;
pub mod translation;
pub mod protocol;
pub mod nbt;
pub mod chunk;
pub mod entity;
pub mod item;
pub mod block;

pub use error::{StarlightError, StarlightResult};
pub use model::canonical::*;
pub use translation::engine::*;

/// Versión del crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Inicializa el logger y subsistema de tracing
pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("starlight_core=info".parse().unwrap())
        )
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_init_logging() {
        // Should not panic
        let _ = tracing_subscriber::fmt()
            .with_env_filter("starlight_core=debug")
            .try_init();
    }
}
