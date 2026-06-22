mod pdf_ops;
mod ocr;

#[tauri::command]
async fn merge_pdfs(input_paths: Vec<String>, output_path: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        pdf_ops::merge_pdfs_impl(input_paths, output_path)
    }).await.map_err(|e| format!("Thread pool join error: {}", e))?
}

#[tauri::command]
async fn split_pdf(input_path: String, output_dir: String, ranges: Vec<(usize, usize)>) -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(move || {
        pdf_ops::split_pdf_impl(input_path, output_dir, ranges)
    }).await.map_err(|e| format!("Thread pool join error: {}", e))?
}

#[tauri::command]
async fn compress_pdf(input_path: String, output_path: String, quality: u8) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        pdf_ops::compress_pdf_impl(input_path, output_path, quality)
    }).await.map_err(|e| format!("Thread pool join error: {}", e))?
}

#[tauri::command]
async fn get_pdf_metadata(filepath: String) -> Result<pdf_ops::PdfMetadata, String> {
    tokio::task::spawn_blocking(move || {
        pdf_ops::get_pdf_metadata_impl(filepath)
    }).await.map_err(|e| format!("Thread pool join error: {}", e))?
}

#[tauri::command]
async fn perform_ocr_command(image_base64: String) -> Result<ocr::OcrResult, String> {

    // Decode base64 string to bytes
    let clean_base64 = if image_base64.contains(",") {
        image_base64.split(",").collect::<Vec<&str>>()[1]
    } else {
        &image_base64
    };
    
    // Using standard base64 decoding. In rust, we can use the `lopdf` or just implement a simple base64 decoder or add the `base64` crate.
    // Wait, lopdf has an internal base64 module, or we can just use the `base64` crate. To be safe, let's add `base64` to Cargo.toml. Let's use `base64` crate, it's very light.
    // Wait, actually `lopdf` doesn't export base64 publicly, so adding base64 crate is much better.
    let bytes = base64::decode(clean_base64).map_err(|e| format!("Base64 decoding failed: {}", e))?;
    ocr::perform_ocr(&bytes).await
}

#[tauri::command]
async fn read_pdf_file(path: String) -> Result<Vec<u8>, String> {
    tokio::task::spawn_blocking(move || {
        pdf_ops::read_pdf_file_impl(path)
    }).await.map_err(|e| format!("Thread pool join error: {}", e))?
}

#[tauri::command]
fn select_save_file(default_name: String) -> Option<String> {
    let dialog = rfd::FileDialog::new()
        .set_file_name(&default_name)
        .add_filter("PDF Document (*.pdf)", &["pdf"]);
    
    dialog.save_file()
        .map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
fn select_directory() -> Option<String> {
    rfd::FileDialog::new()
        .pick_folder()
        .map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
fn open_path(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let clean_path = path.replace("/", "\\");
        let _ = std::process::Command::new("explorer").arg(&clean_path).spawn();
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = std::process::Command::new("open").arg(&path).spawn();
        Ok(())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            merge_pdfs,
            split_pdf,
            compress_pdf,
            perform_ocr_command,
            get_pdf_metadata,
            read_pdf_file,
            select_save_file,
            select_directory,
            open_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
