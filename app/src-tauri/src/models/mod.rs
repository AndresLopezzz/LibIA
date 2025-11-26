// Módulo que contiene todos los modelos de datos de la aplicación

pub mod document;
pub mod chunk;

// Re-exportamos los tipos principales para facilitar su uso
pub use document::Document;
pub use chunk::Chunk;


