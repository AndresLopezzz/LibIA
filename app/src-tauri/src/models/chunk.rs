use serde::{Deserialize, Serialize};

/// Representa un fragmento (chunk) de texto extraído de un documento
///
/// Los documentos se dividen en chunks para facilitar:
/// - Búsqueda semántica (cada chunk puede tener su embedding)
/// - Procesamiento por partes (los LLMs tienen límites de tokens)
/// - Mejor precisión en las respuestas (contexto más específico)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Chunk {
    /// ID único del chunk
    pub id: String,

    /// ID del documento al que pertenece este chunk
    pub document_id: String,

    /// Contenido de texto del chunk
    pub text: String,

    /// Índice/posición del chunk dentro del documento (0-based)
    pub index: usize,

    /// Número de página donde comienza este chunk
    pub page_number: usize,

    /// Número de caracteres en el chunk (útil para validación)
    pub char_count: usize,

    /// Metadata adicional en formato JSON (puede contener info extra)
    pub metadata: Option<String>,
}

impl Chunk {
    /// Crea un nuevo chunk
    ///
    /// # Ejemplo
    /// ```
    /// # use frontend_lib::models::Chunk;
    /// let chunk = Chunk::new(
    ///     "chunk-1".to_string(),
    ///     "doc-123".to_string(),
    ///     "Este es el texto del chunk...".to_string(),
    ///     0,
    ///     1
    /// );
    /// ```
    pub fn new(
        id: String,
        document_id: String,
        text: String,
        index: usize,
        page_number: usize,
    ) -> Self {
        let char_count = text.chars().count();

        Self {
            id,
            document_id,
            text,
            index,
            page_number,
            char_count,
            metadata: None,
        }
    }

    /// Agrega metadata adicional al chunk
    pub fn with_metadata(mut self, metadata: String) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Verifica si el chunk está vacío
    pub fn is_empty(&self) -> bool {
        self.text.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_chunk_creation() {
        let chunk = Chunk::new(
            "chunk-1".to_string(),
            "doc-123".to_string(),
            "Este es un texto de prueba".to_string(),
            0,
            1,
        );

        assert_eq!(chunk.id, "chunk-1");
        assert_eq!(chunk.document_id, "doc-123");
        assert_eq!(chunk.text, "Este es un texto de prueba");
        assert_eq!(chunk.index, 0);
        assert_eq!(chunk.page_number, 1);
        assert_eq!(chunk.char_count, 26); // "Este es un texto de prueba" tiene 26 caracteres
        assert!(chunk.metadata.is_none());
    }

    #[test]
    fn test_chunk_with_metadata() {
        let chunk = Chunk::new(
            "chunk-1".to_string(),
            "doc-123".to_string(),
            "Texto".to_string(),
            0,
            1,
        )
        .with_metadata(r#"{"key": "value"}"#.to_string());

        assert!(chunk.metadata.is_some());
        assert_eq!(chunk.metadata.unwrap(), r#"{"key": "value"}"#);
    }

    #[test]
    fn test_chunk_is_empty() {
        let empty_chunk = Chunk::new(
            "chunk-1".to_string(),
            "doc-123".to_string(),
            "   ".to_string(), // Solo espacios
            0,
            1,
        );

        let non_empty_chunk = Chunk::new(
            "chunk-2".to_string(),
            "doc-123".to_string(),
            "Texto con contenido".to_string(),
            0,
            1,
        );

        assert!(empty_chunk.is_empty());
        assert!(!non_empty_chunk.is_empty());
    }

    #[test]
    fn test_chunk_serialization() {
        let chunk = Chunk::new(
            "chunk-1".to_string(),
            "doc-123".to_string(),
            "Texto del chunk".to_string(),
            5,
            2,
        );

        // Serializar a JSON
        let json = serde_json::to_string(&chunk).expect("Debe serializar correctamente");
        assert!(json.contains("chunk-1"));
        assert!(json.contains("doc-123"));
        assert!(json.contains("Texto del chunk"));

        // Deserializar desde JSON
        let chunk_deserialized: Chunk =
            serde_json::from_str(&json).expect("Debe deserializar correctamente");

        assert_eq!(chunk.id, chunk_deserialized.id);
        assert_eq!(chunk.document_id, chunk_deserialized.document_id);
        assert_eq!(chunk.text, chunk_deserialized.text);
    }

    #[test]
    fn test_chunk_roundtrip() {
        let original = Chunk::new(
            "chunk-123".to_string(),
            "doc-456".to_string(),
            "Contenido del chunk con texto más largo".to_string(),
            10,
            5,
        )
        .with_metadata(r#"{"extra": "data"}"#.to_string());

        // Serializar y deserializar
        let json = serde_json::to_string(&original).unwrap();
        let restored: Chunk = serde_json::from_str(&json).unwrap();

        // Verificar que todos los campos coinciden
        assert_eq!(original.id, restored.id);
        assert_eq!(original.document_id, restored.document_id);
        assert_eq!(original.text, restored.text);
        assert_eq!(original.index, restored.index);
        assert_eq!(original.page_number, restored.page_number);
        assert_eq!(original.char_count, restored.char_count);
        assert_eq!(original.metadata, restored.metadata);
    }

    #[test]
    fn test_chunk_char_count() {
        // Test con texto simple
        let chunk1 = Chunk::new(
            "chunk-1".to_string(),
            "doc-1".to_string(),
            "Hola".to_string(),
            0,
            1,
        );
        assert_eq!(chunk1.char_count, 4);

        // Test con texto más largo
        let chunk2 = Chunk::new(
            "chunk-2".to_string(),
            "doc-1".to_string(),
            "Este es un texto más largo con múltiples palabras".to_string(),
            0,
            1,
        );
        assert_eq!(chunk2.char_count, 49); // "Este es un texto más largo con múltiples palabras" tiene 49 caracteres

        // Test con caracteres especiales (UTF-8)
        let chunk3 = Chunk::new(
            "chunk-3".to_string(),
            "doc-1".to_string(),
            "Hola ñoño".to_string(),
            0,
            1,
        );
        assert_eq!(chunk3.char_count, 9); // Incluye espacios y ñ
    }
}
