use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Representa un documento PDF cargado en el sistema
///
/// Este struct almacena la información básica de un documento:
/// - Identificador único
/// - Nombre del archivo
/// - Fecha de carga
/// - Número de páginas
/// - Estado de indexación (si ya tiene embeddings generados)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Document {
    /// ID único del documento (usualmente UUID o hash del archivo)
    pub id: String,

    /// Nombre original del archivo PDF
    pub name: String,

    /// Ruta completa del archivo en el sistema
    pub file_path: String,

    /// Número de páginas del documento
    pub page_count: usize,

    /// Fecha de carga en formato timestamp Unix
    pub created_at: u64,

    /// Indica si el documento ya fue indexado (tiene embeddings generados)
    pub is_indexed: bool,
}

impl Document {
    /// Crea un nuevo documento con la fecha actual
    ///
    /// # Ejemplo
    /// ```
    /// # use frontend_lib::models::Document;
    /// let doc = Document::new(
    ///     "doc-123".to_string(),
    ///     "mi_documento.pdf".to_string(),
    ///     "/ruta/al/archivo.pdf".to_string(),
    ///     10
    /// );
    /// ```
    pub fn new(id: String, name: String, file_path: String, page_count: usize) -> Self {
        let created_at = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id,
            name,
            file_path,
            page_count,
            created_at,
            is_indexed: false,
        }
    }

    /// Marca el documento como indexado
    pub fn mark_as_indexed(&mut self) {
        self.is_indexed = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_document_creation() {
        let doc = Document::new(
            "test-id".to_string(),
            "test.pdf".to_string(),
            "/path/to/test.pdf".to_string(),
            5,
        );

        assert_eq!(doc.id, "test-id");
        assert_eq!(doc.name, "test.pdf");
        assert_eq!(doc.page_count, 5);
        assert!(!doc.is_indexed);
        assert!(doc.created_at > 0);
    }

    #[test]
    fn test_document_mark_as_indexed() {
        let mut doc = Document::new(
            "test-id".to_string(),
            "test.pdf".to_string(),
            "/path/to/test.pdf".to_string(),
            5,
        );

        assert!(!doc.is_indexed);
        doc.mark_as_indexed();
        assert!(doc.is_indexed);
    }

    #[test]
    fn test_document_serialization() {
        let doc = Document::new(
            "test-id".to_string(),
            "test.pdf".to_string(),
            "/path/to/test.pdf".to_string(),
            5,
        );

        // Serializar a JSON
        let json = serde_json::to_string(&doc).expect("Debe serializar correctamente");
        assert!(json.contains("test-id"));
        assert!(json.contains("test.pdf"));

        // Deserializar desde JSON
        let doc_deserialized: Document =
            serde_json::from_str(&json).expect("Debe deserializar correctamente");

        assert_eq!(doc.id, doc_deserialized.id);
        assert_eq!(doc.name, doc_deserialized.name);
        assert_eq!(doc.page_count, doc_deserialized.page_count);
    }

    #[test]
    fn test_document_roundtrip() {
        let original = Document::new(
            "doc-123".to_string(),
            "documento.pdf".to_string(),
            "/ruta/documento.pdf".to_string(),
            10,
        );

        // Serializar y deserializar
        let json = serde_json::to_string(&original).unwrap();
        let restored: Document = serde_json::from_str(&json).unwrap();

        // Verificar que todos los campos coinciden
        assert_eq!(original.id, restored.id);
        assert_eq!(original.name, restored.name);
        assert_eq!(original.file_path, restored.file_path);
        assert_eq!(original.page_count, restored.page_count);
        assert_eq!(original.created_at, restored.created_at);
        assert_eq!(original.is_indexed, restored.is_indexed);
    }
}
