/// Читает URDF-файл по абсолютному пути и возвращает XML-строку.
/// Путь получается от фронтенда после dialog::open().
#[tauri::command]
pub async fn read_urdf_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| format!("Cannot read {path}: {e}"))
}
