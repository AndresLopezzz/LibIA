use crate::models::Document;
use bincode;
use sled;
use std::{fs, path::PathBuf, sync::Arc};

fn default_app_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

pub fn get_db_dir(app_name: Option<&str>) -> PathBuf {
    let app_name = app_name.unwrap_or(default_app_name());

    // Usamos dirs::data_local_dir() que es multiplataforma
    // Retorna el directorio de datos local del usuario
    let mut base = dirs::data_local_dir()
        .or_else(|| dirs::data_dir())
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    base.push(app_name);
    base
}

pub fn get_db_path(app_name: Option<&str>, db_subdir: Option<&str>) -> Result<PathBuf, String> {
    let mut dir = get_db_dir(app_name);
    let sub = db_subdir.unwrap_or("sled_db");
    dir.push(sub);
    fs::create_dir_all(&dir).map_err(|e| format!("failed to create db dir: {}", e))?;
    Ok(dir)
}

pub fn init_db(app_name: Option<&str>, db_subdir: Option<&str>) -> Result<Arc<sled::Db>, String> {
    let db_dir = get_db_path(app_name, db_subdir)?;
    let db = sled::open(&db_dir).map_err(|e| format!("failed to open sled db: {}", e))?;
    Ok(Arc::new(db))
}

fn open_documents_tree(db: &sled::Db) -> Result<sled::Tree, String> {
    db.open_tree("documents")
        .map_err(|e| format!("failed to open documents tree: {}", e))
}

pub fn insert_document(db: &Arc<sled::Db>, doc: &Document) -> Result<(), String> {
    let tree = open_documents_tree(&*db)?;
    let v = bincode::serialize(doc).map_err(|e| format!("serialize error: {}", e))?;
    tree.insert(doc.id.as_bytes(), v)
        .map_err(|e| format!("sled insert error: {}", e))?;
    tree.flush().map_err(|e| format!("flush error: {}", e))?;
    Ok(())
}

pub fn get_document(db: &Arc<sled::Db>, id: &str) -> Result<Option<Document>, String> {
    let tree = open_documents_tree(&*db)?;
    match tree
        .get(id.as_bytes())
        .map_err(|e| format!("sled get error: {}", e))?
    {
        Some(bytes) => {
            let doc: Document =
                bincode::deserialize(&bytes).map_err(|e| format!("deseralization error: {}", e))?;
            Ok(Some(doc))
        }
        None => Ok(None),
    }
}

pub fn get_all_documents(db: &Arc<sled::Db>) -> Result<Vec<Document>, String> {
    let tree = open_documents_tree(&*db)?;
    let mut out = Vec::new();
    for item in tree.iter() {
        let (_k, v) = item.map_err(|e| format!("sled iter error: {}", e))?;
        let doc: Document =
            bincode::deserialize(&v).map_err(|e| format!("desearialize error: {}", e))?;
        out.push(doc);
    }
    Ok(out)
}

pub fn delete_document(db: &Arc<sled::Db>, id: &str) -> Result<(), String> {
    let tree = open_documents_tree(&*db)?;
    tree.remove(id.as_bytes())
        .map_err(|e| format!("sled remove error: {}", e))?;
    tree.flush().map_err(|e| format!("flush error: {}", e))?;
    Ok(())
}

// TEST -------------------------------------------- TEST

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_get_db_dir() {
        let dir = get_db_dir(None);

        // Verificar que el path existe o puede ser creado
        assert!(!dir.as_os_str().is_empty());

        // Verificar que contiene el nombre de la app
        let dir_str = dir.to_string_lossy();
        assert!(dir_str.contains("libAi") || dir_str.contains("LibAI"));
    }

    #[test]
    fn test_get_db_dir_custom_app_name() {
        let custom_name = "test_app";
        let dir = get_db_dir(Some(custom_name));
        let dir_str = dir.to_string_lossy();

        // Verificar que contiene el nombre personalizado
        assert!(dir_str.contains(custom_name));
    }

    #[test]
    fn test_get_db_path() {
        // Usar un nombre de app único para tests
        let test_app = format!("test_libai_{}", std::process::id());
        let result = get_db_path(Some(&test_app), Some("test_db"));

        assert!(result.is_ok());
        let path = result.unwrap();

        // Verificar que el directorio fue creado
        assert!(path.exists(), "El directorio de BD debe existir");
        assert!(path.is_dir(), "El path debe ser un directorio");

        // Limpiar después del test
        let _ = fs::remove_dir_all(&path);
    }

    #[test]
    fn test_get_db_path_default_subdir() {
        let test_app = format!("test_libai_default_{}", std::process::id());
        let result = get_db_path(Some(&test_app), None);

        assert!(result.is_ok());
        let path = result.unwrap();

        // Verificar que el subdirectorio por defecto es "sled_db"
        assert!(path.ends_with("sled_db") || path.to_string_lossy().contains("sled_db"));
        assert!(path.exists());

        // Limpiar después del test
        let _ = fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn test_init_db() {
        // Usar un nombre único para cada test
        let test_app = format!("test_libai_init_{}", std::process::id());
        let test_subdir = format!("test_init_db_{}", std::process::id());

        // Inicializar la BD
        let db_result = init_db(Some(&test_app), Some(&test_subdir));
        assert!(db_result.is_ok(), "init_db debe retornar Ok");

        let db = db_result.unwrap();

        // Verificar que la BD está abierta (podemos hacer operaciones básicas)
        // Intentar insertar y leer un valor de prueba
        let test_key = b"test_key";
        let test_value = b"test_value";

        let insert_result = db.insert(test_key, test_value);
        assert!(insert_result.is_ok(), "Debe poder insertar en la BD");

        // Leer el valor insertado
        let read_result = db.get(test_key);
        assert!(read_result.is_ok(), "Debe poder leer de la BD");

        let retrieved = read_result.unwrap();
        assert!(retrieved.is_some(), "Debe encontrar el valor insertado");
        assert_eq!(retrieved.unwrap().as_ref(), test_value);

        // Limpiar: eliminar el test key
        let _ = db.remove(test_key);

        // Verificar que el directorio de BD existe en disco
        let db_path = get_db_path(Some(&test_app), Some(&test_subdir)).unwrap();
        assert!(
            db_path.exists(),
            "El directorio de BD debe existir en disco"
        );

        // Limpiar después del test
        let _ = fs::remove_dir_all(&db_path);
    }

    #[test]
    fn test_init_db_open_and_close() {
        let test_app = format!("test_libai_openclose_{}", std::process::id());
        let test_subdir = format!("test_openclose_{}", std::process::id());

        // Abrir la BD
        let db1_result = init_db(Some(&test_app), Some(&test_subdir));
        assert!(db1_result.is_ok());

        let db1 = db1_result.unwrap();

        // Insertar datos
        let _ = db1.insert(b"key1", b"value1");
        let _ = db1.insert(b"key2", b"value2");

        // Cerrar la BD (drop)
        drop(db1);

        // Reabrir la BD (debe persistir los datos)
        let db2_result = init_db(Some(&test_app), Some(&test_subdir));
        assert!(db2_result.is_ok());

        let db2 = db2_result.unwrap();

        // Verificar que los datos persisten
        let value1 = db2.get(b"key1").unwrap();
        assert!(value1.is_some());
        assert_eq!(value1.unwrap().as_ref(), b"value1");

        let value2 = db2.get(b"key2").unwrap();
        assert!(value2.is_some());
        assert_eq!(value2.unwrap().as_ref(), b"value2");

        // Limpiar
        let db_path = get_db_path(Some(&test_app), Some(&test_subdir)).unwrap();
        let _ = fs::remove_dir_all(&db_path);
    }

    #[test]
    fn test_db_path_correct_for_os() {
        let test_app = "test_os_path";
        let path = get_db_path(Some(test_app), Some("test")).unwrap();
        let path_str = path.to_string_lossy().to_lowercase();

        // Verificar que el path es correcto según el OS
        #[cfg(windows)]
        {
            // En Windows debería estar en LocalAppData
            assert!(
                path_str.contains("appdata") || path_str.contains("local"),
                "En Windows debe estar en AppData\\Local"
            );
        }

        #[cfg(target_os = "macos")]
        {
            // En macOS debería estar en ~/Library/Application Support
            assert!(
                path_str.contains("library") || path_str.contains("application support"),
                "En macOS debe estar en ~/Library/Application Support"
            );
        }

        #[cfg(target_os = "linux")]
        {
            // En Linux debería estar en ~/.local/share
            assert!(
                path_str.contains(".local") || path_str.contains("share"),
                "En Linux debe estar en ~/.local/share"
            );
        }

        // Limpiar
        let _ = fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn test_multiple_db_instances() {
        let test_app = format!("test_multi_{}", std::process::id());
        let test_subdir = format!("test_multi_db_{}", std::process::id());

        // Nota: Sled no permite abrir múltiples instancias de la misma BD simultáneamente
        // debido a locks de archivo. Este test verifica que podemos usar Arc para compartir
        // una única instancia entre múltiples referencias.

        // Crear una instancia de BD
        let db1 = init_db(Some(&test_app), Some(&test_subdir)).unwrap();

        // Clonar la referencia Arc (no crea una nueva BD, solo otra referencia)
        let db2 = Arc::clone(&db1);

        // Insertar en una referencia
        let _ = db1.insert(b"shared_key", b"shared_value");

        // Leer desde la otra referencia (debe ver los mismos datos)
        let value = db2.get(b"shared_key").unwrap();
        assert!(value.is_some());
        assert_eq!(value.unwrap().as_ref(), b"shared_value");

        // Limpiar
        drop(db1);
        drop(db2);
        let db_path = get_db_path(Some(&test_app), Some(&test_subdir)).unwrap();
        let _ = fs::remove_dir_all(&db_path);
    }

    #[test]
    fn test_insert_and_get_document_minimal() {
        let test_app = format!("test_insert_{}", std::process::id());
        let test_sub = format!("test_db_{}", std::process::id());
        let db = init_db(Some(&test_app), Some(&test_sub)).unwrap();

        // Crear documento
        let doc = Document::new(
            "doc-1".to_string(),
            "prueba.pdf".to_string(),
            "/tmp/prueba.pdf".to_string(),
            5,
        );

        assert!(insert_document(&db, &doc).is_ok());

        // Cleanup
        let db_path = get_db_path(Some(&test_app), Some(&test_sub)).unwrap();
        let _ = std::fs::remove_dir_all(&db_path);

        let got = get_document(&db, &doc.id).unwrap();
        assert!(got.is_some());
        let got_doc = got.unwrap();
        assert_eq!(got_doc.id, doc.id);
        assert_eq!(got_doc.name, doc.name);
    }

    #[test]
    fn test_get_all_documents() {
        let test_app = format!("test_get_all_{}", std::process::id());
        let test_sub = format!("test_get_all_db_{}", std::process::id());
        let db = init_db(Some(&test_app), Some(&test_sub)).unwrap();

        let d1 = Document::new(
            "d1".to_string(),
            "a.pdf".to_string(),
            "/tmp/a.pdf".to_string(),
            1,
        );
        let d2 = Document::new(
            "d2".to_string(),
            "b.pdf".to_string(),
            "/tmp/b.pdf".to_string(),
            1,
        );

        insert_document(&db, &d1).unwrap();
        insert_document(&db, &d2).unwrap();

        let all = get_all_documents(&db).unwrap();
        let ids: Vec<String> = all.into_iter().map(|d| d.id).collect();
        assert!(ids.contains(&"d1".to_string()));
        assert!(ids.contains(&"d2".to_string()));

        let db_path = get_db_path(Some(&test_app), Some(&test_sub)).unwrap();
        let _ = std::fs::remove_dir_all(&db_path);
    }
}
