/*!
 * Tauri Commands Module
 * 
 * This module contains all the backend commands that can be invoked from the frontend.
 * It handles file system operations, Excel processing, and folder renaming operations
 * with support for Azerbaijani alphabet sorting and process control.
 */

use calamine::{open_workbook, DataType, Reader, Xlsx};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use tauri::{command, Window, State};
use tokio::time::sleep;

#[cfg(windows)]
use windows::core::PCWSTR;
#[cfg(windows)]
use windows::Win32::UI::Shell::StrCmpLogicalW;

// ================================================================================================
// Global Process State
// ================================================================================================

#[derive(Default)]
pub struct ProcessState {
    pub is_running: AtomicBool,
    pub is_paused: AtomicBool,
    pub should_stop: AtomicBool,
    pub current_index: AtomicUsize,
}

impl ProcessState {
    pub fn new() -> Self {
        Self {
            is_running: AtomicBool::new(false),
            is_paused: AtomicBool::new(false),
            should_stop: AtomicBool::new(false),
            current_index: AtomicUsize::new(0),
        }
    }

    pub fn start(&self) {
        self.is_running.store(true, Ordering::Relaxed);
        self.is_paused.store(false, Ordering::Relaxed);
        self.should_stop.store(false, Ordering::Relaxed);
        self.current_index.store(0, Ordering::Relaxed);
    }

    pub fn pause(&self) {
        self.is_paused.store(true, Ordering::Relaxed);
    }

    pub fn resume(&self) {
        self.is_paused.store(false, Ordering::Relaxed);
    }

    pub fn stop(&self) {
        self.should_stop.store(true, Ordering::Relaxed);
        self.is_running.store(false, Ordering::Relaxed);
        self.is_paused.store(false, Ordering::Relaxed);
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused.load(Ordering::Relaxed)
    }

    pub fn should_stop(&self) -> bool {
        self.should_stop.load(Ordering::Relaxed)
    }

    pub fn reset(&self) {
        self.is_running.store(false, Ordering::Relaxed);
        self.is_paused.store(false, Ordering::Relaxed);
        self.should_stop.store(false, Ordering::Relaxed);
        self.current_index.store(0, Ordering::Relaxed);
    }
}

// ================================================================================================
// Data Structures
// ================================================================================================

/// Represents file or folder information
#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size: u64,
    pub extension: Option<String>,
}

/// Progress update structure for real-time feedback
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProgressUpdate {
    pub current: usize,
    pub total: usize,
    pub percentage: f32,
    pub current_step: String,
    pub message: String,
}

/// Result of a folder processing operation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessResult {
    pub success: bool,
    pub message: String,
    pub folder_name: String,
    pub new_name: String,
}

// ================================================================================================
// Process Control Commands
// ================================================================================================

/// Pauses the current process
#[command]
pub fn pause_process(state: State<ProcessState>) -> Result<(), String> {
    if state.is_running() {
        state.pause();
        Ok(())
    } else {
        Err("Proses işləmir".to_string())
    }
}

/// Resumes the paused process
#[command]
pub fn resume_process(state: State<ProcessState>) -> Result<(), String> {
    if state.is_running() && state.is_paused() {
        state.resume();
        Ok(())
    } else {
        Err("Proses fasilədə deyil".to_string())
    }
}

/// Stops the current process
#[command]
pub fn stop_process(state: State<ProcessState>) -> Result<(), String> {
    if state.is_running() {
        state.stop();
        Ok(())
    } else {
        Err("Proses işləmir".to_string())
    }
}

/// Gets the current process status
#[command]
pub fn get_process_status(state: State<ProcessState>) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "is_running": state.is_running(),
        "is_paused": state.is_paused(),
        "should_stop": state.should_stop()
    }))
}

// ================================================================================================
// Basic Commands
// ================================================================================================

/// Simple greeting command for testing backend connectivity
#[command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// ================================================================================================
// PDF Creation Commands
// ================================================================================================

/// Represents PDF creation configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct PdfConfig {
    pub main_folder: String,
    pub subfolder_name: String,
    pub delete_files: Vec<String>,
}

/// Represents the result of PDF creation for a single folder
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PdfResult {
    pub success: bool,
    pub folder_name: String,
    pub message: String,
    pub images_found: usize,
    pub pdf_created: bool,
}

/// Creates PDF files from images in subfolders with process control
#[command]
pub async fn create_pdf_from_images(
    window: Window,
    config: PdfConfig,
    state: State<'_, ProcessState>,
) -> Result<Vec<PdfResult>, String> {
    // Start the process
    state.start();
    
    let main_folder = Path::new(&config.main_folder);
    if !main_folder.exists() {
        return Err("Əsas qovluq mövcud deyil".to_string());
    }

    let mut results = Vec::new();
    let mut subfolders = Vec::new();

    // Collect all subfolders
    match fs::read_dir(main_folder) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        subfolders.push(entry.file_name().to_string_lossy().to_string());
                    }
                }
            }
        }
        Err(e) => {
            state.reset();
            return Err(format!("Qovluq oxunması xətası: {}", e));
        }
    }

    if subfolders.is_empty() {
        state.reset();
        return Err("Alt qovluqlar tapılmadı".to_string());
    }

    let total_folders = subfolders.len();

    // Process each subfolder
    for (index, folder_name) in subfolders.iter().enumerate() {
        // Check if process should stop
        if state.should_stop() {
            break;
        }

        // Handle pause
        while state.is_paused() && !state.should_stop() {
            sleep(Duration::from_millis(100)).await;
        }

        if state.should_stop() {
            break;
        }

        let folder_path = main_folder.join(folder_name);
        let subfolder_path = folder_path.join(&config.subfolder_name);

        // Emit progress
        emit_progress(
            &window,
            index + 1,
            total_folders,
            &format!("'{}' qovluğu işlənir", folder_name),
            &format!("{}/{} qovluq", index + 1, total_folders),
        );

        let result = if subfolder_path.exists() && subfolder_path.is_dir() {
            match process_folder_for_pdf(&folder_path, &subfolder_path, &config.subfolder_name, &config.delete_files).await {
                Ok(images_count) => {
                    emit_process_result(&window, true, &format!("PDF yaradıldı: {}_picture.pdf", folder_name), folder_name, "");
                    PdfResult {
                        success: true,
                        folder_name: folder_name.clone(),
                        message: format!("PDF uğurla yaradıldı ({} şəkil)", images_count),
                        images_found: images_count,
                        pdf_created: true,
                    }
                }
                Err(e) => {
                    emit_process_result(&window, false, &format!("Xəta: {}", e), folder_name, "");
                    PdfResult {
                        success: false,
                        folder_name: folder_name.clone(),
                        message: format!("Xəta: {}", e),
                        images_found: 0,
                        pdf_created: false,
                    }
                }
            }
        } else {
            PdfResult {
                success: false,
                folder_name: folder_name.clone(),
                message: format!("'{}' alt qovluğu tapılmadı", config.subfolder_name),
                images_found: 0,
                pdf_created: false,
            }
        };

        results.push(result);

        // Small delay for UI updates
        sleep(Duration::from_millis(50)).await;
    }

    // Clean up empty directories
    if let Err(e) = remove_empty_directories(main_folder) {
        eprintln!("Boş qovluqları silmə xətası: {}", e);
    }

    state.reset();
    Ok(results)
}

/// Gets list of subfolders in the main directory for PDF processing
#[command]
pub async fn get_pdf_subfolders(main_folder: String) -> Result<Vec<FileInfo>, String> {
    let main_path = Path::new(&main_folder);
    
    if !main_path.exists() {
        return Err("Qovluq mövcud deyil".to_string());
    }

    let mut subfolders = Vec::new();
    
    match fs::read_dir(main_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let metadata = entry.metadata().map_err(|e| e.to_string())?;
                    
                    if metadata.is_dir() {
                        // Check if this subfolder contains image subfolder
                        let subfolder_path = path.join("images"); // Default check
                        let has_images = subfolder_path.exists() && 
                            has_image_files(&subfolder_path).unwrap_or(false);
                        
                        let file_info = FileInfo {
                            name: entry.file_name().to_string_lossy().to_string(),
                            path: path.to_string_lossy().to_string(),
                            is_directory: true,
                            size: if has_images { 1 } else { 0 }, // Use size field to indicate if has images
                            extension: None,
                        };
                        
                        subfolders.push(file_info);
                    }
                }
            }
        }
        Err(e) => return Err(e.to_string()),
    }
    
    // Sort alphabetically
    subfolders.sort_by(|a, b| natural_sort_compare(&a.name, &b.name));
    
    Ok(subfolders)
}

// ================================================================================================
// File System Operations
// ================================================================================================

/// Retrieves all files in a specified directory
#[command]
pub async fn get_files_in_directory(path: String) -> Result<Vec<FileInfo>, String> {
    let dir_path = Path::new(&path);
    
    if !dir_path.exists() {
        return Err("Qovluq mövcud deyil".to_string());
    }

    let mut files = Vec::new();
    
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let metadata = entry.metadata().map_err(|e| e.to_string())?;
                    
                    let file_info = FileInfo {
                        name: entry.file_name().to_string_lossy().to_string(),
                        path: path.to_string_lossy().to_string(),
                        is_directory: metadata.is_dir(),
                        size: metadata.len(),
                        extension: path.extension().map(|ext| ext.to_string_lossy().to_string()),
                    };
                    
                    files.push(file_info);
                }
            }
        }
        Err(e) => return Err(e.to_string()),
    }
    
    Ok(files)
}

/// Retrieves all folders in a specified directory without sorting
#[command]
pub async fn get_folders_in_directory(path: String) -> Result<Vec<FileInfo>, String> {
    let dir_path = Path::new(&path);
    
    if !dir_path.exists() {
        return Err("Qovluq mövcud deyil".to_string());
    }

    let mut folders = Vec::new();
    
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let metadata = entry.metadata().map_err(|e| e.to_string())?;
                    
                    if metadata.is_dir() {
                        let file_info = FileInfo {
                            name: entry.file_name().to_string_lossy().to_string(),
                            path: path.to_string_lossy().to_string(),
                            is_directory: true,
                            size: 0,
                            extension: None,
                        };
                        
                        folders.push(file_info);
                    }
                }
            }
        }
        Err(e) => return Err(e.to_string()),
    }
    
    Ok(folders)
}

/// Retrieves folders with specified sorting method
#[command]
pub async fn get_folders_with_sorting(
    path: String,
    sort_order: String,
) -> Result<Vec<FileInfo>, String> {
    let dir_path = Path::new(&path);
    
    if !dir_path.exists() {
        return Err("Qovluq mövcud deyil".to_string());
    }

    let mut folders = Vec::new();
    
    // Collect all folder entries
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let metadata = entry.metadata().map_err(|e| e.to_string())?;
                    
                    if metadata.is_dir() {
                        let file_info = FileInfo {
                            name: entry.file_name().to_string_lossy().to_string(),
                            path: path.to_string_lossy().to_string(),
                            is_directory: true,
                            size: 0,
                            extension: None,
                        };
                        
                        folders.push(file_info);
                    }
                }
            }
        }
        Err(e) => return Err(e.to_string()),
    }
    
    // Apply sorting based on user selection
    match sort_order.as_str() {
        "name" => {
            folders.sort_by(|a, b| natural_sort_compare(&a.name, &b.name));
        }
        "date" => {
            folders.sort_by(|a, b| {
                let a_metadata = std::fs::metadata(&a.path).ok();
                let b_metadata = std::fs::metadata(&b.path).ok();
                
                match (a_metadata, b_metadata) {
                    (Some(a_meta), Some(b_meta)) => {
                        let a_time = a_meta.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                        let b_time = b_meta.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                        b_time.cmp(&a_time) // Newest first
                    }
                    _ => std::cmp::Ordering::Equal,
                }
            });
        }
        "size" => {
            folders.sort_by(|a, b| {
                let a_size = get_folder_size(&a.path).unwrap_or(0);
                let b_size = get_folder_size(&b.path).unwrap_or(0);
                b_size.cmp(&a_size) // Largest first
            });
        }
        _ => {
            // Default: natural sort (like Windows Explorer)
            folders.sort_by(|a, b| natural_sort_compare(&a.name, &b.name));
        }
    }
    
    Ok(folders)
}

/// Retrieves files with specified sorting method
#[command]
pub async fn get_files_with_sorting(
    path: String,
    sort_order: String,
) -> Result<Vec<FileInfo>, String> {
    let dir_path = Path::new(&path);
    
    if !dir_path.exists() {
        return Err("Qovluq mövcud deyil".to_string());
    }

    let mut files = Vec::new();
    
    // Collect all file entries
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let metadata = entry.metadata().map_err(|e| e.to_string())?;
                    
                    if metadata.is_file() {
                        let file_info = FileInfo {
                            name: entry.file_name().to_string_lossy().to_string(),
                            path: path.to_string_lossy().to_string(),
                            is_directory: false,
                            size: metadata.len(),
                            extension: path.extension().map(|ext| ext.to_string_lossy().to_string()),
                        };
                        
                        files.push(file_info);
                    }
                }
            }
        }
        Err(e) => return Err(e.to_string()),
    }
    
    // Apply sorting based on user selection
    match sort_order.as_str() {
        "name" => {
            files.sort_by(|a, b| natural_sort_compare(&a.name, &b.name));
        }
        "date" => {
            files.sort_by(|a, b| {
                let a_metadata = std::fs::metadata(&a.path).ok();
                let b_metadata = std::fs::metadata(&b.path).ok();
                
                match (a_metadata, b_metadata) {
                    (Some(a_meta), Some(b_meta)) => {
                        let a_time = a_meta.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                        let b_time = b_meta.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                        b_time.cmp(&a_time) // Newest first
                    }
                    _ => std::cmp::Ordering::Equal,
                }
            });
        }
        "size" => {
            files.sort_by(|a, b| b.size.cmp(&a.size)); // Largest first
        }
        _ => {
            // Default: natural sort (like Windows Explorer)
            files.sort_by(|a, b| natural_sort_compare(&a.name, &b.name));
        }
    }
    
    Ok(files)
}

// ================================================================================================
// Renaming Operations
// ================================================================================================

/// Renames files based on pattern matching
#[command]
pub async fn rename_files(
    directory: String,
    pattern: String,
    replacement: String,
) -> Result<Vec<String>, String> {
    let dir_path = Path::new(&directory);
    
    if !dir_path.exists() {
        return Err("Qovluq mövcud deyil".to_string());
    }

    let mut renamed_files = Vec::new();
    
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    
                    if path.is_file() {
                        let old_name = entry.file_name().to_string_lossy().to_string();
                        let new_name = old_name.replace(&pattern, &replacement);
                        
                        if old_name != new_name {
                            let new_path = dir_path.join(&new_name);
                            
                            match fs::rename(&path, &new_path) {
                                Ok(_) => {
                                    renamed_files.push(format!("{} -> {}", old_name, new_name));
                                }
                                Err(e) => {
                                    return Err(format!("Fayl adını dəyişmək mümkün olmadı {}: {}", old_name, e));
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => return Err(e.to_string()),
    }
    
    Ok(renamed_files)
}

/// Renames folders based on pattern matching
#[command]
pub async fn rename_folders(
    directory: String,
    pattern: String,
    replacement: String,
) -> Result<Vec<String>, String> {
    let dir_path = Path::new(&directory);
    
    if !dir_path.exists() {
        return Err("Qovluq mövcud deyil".to_string());
    }

    let mut renamed_folders = Vec::new();
    
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    
                    if path.is_dir() {
                        let old_name = entry.file_name().to_string_lossy().to_string();
                        let new_name = old_name.replace(&pattern, &replacement);
                        
                        if old_name != new_name {
                            let new_path = dir_path.join(&new_name);
                            
                            match fs::rename(&path, &new_path) {
                                Ok(_) => {
                                    renamed_folders.push(format!("{} -> {}", old_name, new_name));
                                }
                                Err(e) => {
                                    return Err(format!("Qovluq adını dəyişmək mümkün olmadı {}: {}", old_name, e));
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => return Err(e.to_string()),
    }
    
    Ok(renamed_folders)
}

/// Main folder renaming operation using Excel data with process control
#[command]
pub async fn rename_folders_from_excel(
    window: Window,
    source_path: String,
    destination_path: String,
    excel_path: String,
    start_row: u32,
    column: String,
    _sort_order: String,
    folders: Vec<String>,
    state: State<'_, ProcessState>,
) -> Result<Vec<String>, String> {
    let source_dir = Path::new(&source_path);
    let dest_dir = Path::new(&destination_path);
    
    // Validate directories
    if !source_dir.exists() {
        return Err("Əsas qovluq mövcud deyil".to_string());
    }
    
    if !dest_dir.exists() {
        return Err("Təyinat qovluq mövcud deyil".to_string());
    }
    
    // Start the process
    state.start();
    
    // Send initial progress
    emit_progress(&window, 0, folders.len(), "Excel faylı oxunur...", "Excel-dən adlar yüklənir");
    
    // Read names from Excel file
    let excel_names = match read_excel_names(&excel_path, start_row, &column) {
        Ok(names) => names,
        Err(e) => {
            state.reset();
            return Err(e);
        }
    };
    
    if excel_names.is_empty() {
        state.reset();
        return Err("Excel faylında heç bir ad tapılmadı".to_string());
    }
    
    emit_progress(&window, 0, folders.len(), "Proses başlanır...", &format!("{} qovluq işlənəcək", folders.len()));
    
    let mut results = Vec::new();
    
    // Process each folder with corresponding Excel name
    for (index, folder_name) in folders.iter().enumerate() {
        // Check if process should stop
        if state.should_stop() {
            emit_progress(&window, index, folders.len(), "Dayandırıldı", "Proses dayandırıldı");
            state.reset();
            return Ok(results);
        }
        
        // Handle pause
        while state.is_paused() && !state.should_stop() {
            emit_progress(&window, index, folders.len(), "Fasilə verildi", "Proses fasilədədir");
            sleep(Duration::from_millis(100)).await;
        }
        
        // Check again after pause
        if state.should_stop() {
            emit_progress(&window, index, folders.len(), "Dayandırıldı", "Proses dayandırıldı");
            state.reset();
            return Ok(results);
        }
        
        let current = index + 1;
        state.current_index.store(current, Ordering::Relaxed);
        
        emit_progress(&window, current, folders.len(), &format!("İşlənən qovluq: {}", folder_name), &format!("{}/{} qovluq", current, folders.len()));
        
        let old_folder_path = source_dir.join(folder_name);
        
        if !old_folder_path.exists() {
            let error_msg = format!("❌ Xəta: '{}' qovluğu tapılmadı", folder_name);
            results.push(error_msg.clone());
            
            emit_process_result(&window, false, &error_msg, folder_name, "");
            continue;
        }
        
        // Get new name from Excel
        let new_name = if index < excel_names.len() {
            &excel_names[index]
        } else {
            let error_msg = format!("❌ Xəta: '{}' qovluğu üçün Excel adı yoxdur (sətir {})", folder_name, start_row + index as u32);
            results.push(error_msg.clone());
            
            emit_process_result(&window, false, &error_msg, folder_name, "");
            continue;
        };
        
        // Create safe filename
        let safe_new_name = sanitize_filename(new_name);
        let new_folder_path = dest_dir.join(&safe_new_name);
        
        // Add delay to show progress
        sleep(Duration::from_millis(500)).await;
        
        // Move and rename folder
        match move_folder(&old_folder_path, &new_folder_path) {
            Ok(_) => {
                let success_msg = format!("✅ Uğur: '{}' → '{}'", folder_name, safe_new_name);
                results.push(success_msg.clone());
                
                emit_process_result(&window, true, &success_msg, folder_name, &safe_new_name);
            }
            Err(e) => {
                let error_msg = format!("❌ Xəta: '{}' köçürülə bilmədi: {}", folder_name, e);
                results.push(error_msg.clone());
                
                emit_process_result(&window, false, &error_msg, folder_name, &safe_new_name);
            }
        }
    }
    
    // Send completion
    if !state.should_stop() {
        emit_progress(&window, folders.len(), folders.len(), "Tamamlandı!", "Bütün qovluqlar işləndi");
    }
    
    state.reset();
    Ok(results)
}

/// Main file renaming operation using Excel data with process control
#[command]
pub async fn rename_files_from_excel(
    window: Window,
    source_path: String,
    destination_path: String,
    excel_path: String,
    start_row: u32,
    column: String,
    _sort_order: String,
    files: Vec<String>,
    state: State<'_, ProcessState>,
) -> Result<Vec<String>, String> {
    let source_dir = Path::new(&source_path);
    let dest_dir = Path::new(&destination_path);
    
    // Validate directories
    if !source_dir.exists() {
        return Err("Əsas qovluq mövcud deyil".to_string());
    }
    
    if !dest_dir.exists() {
        return Err("Təyinat qovluq mövcud deyil".to_string());
    }
    
    // Start the process
    state.start();
    
    // Send initial progress
    emit_progress(&window, 0, files.len(), "Excel faylı oxunur...", "Excel-dən adlar yüklənir");
    
    // Read names from Excel file
    let excel_names = match read_excel_names(&excel_path, start_row, &column) {
        Ok(names) => names,
        Err(e) => {
            state.reset();
            return Err(e);
        }
    };
    
    if excel_names.is_empty() {
        state.reset();
        return Err("Excel faylında heç bir ad tapılmadı".to_string());
    }
    
    emit_progress(&window, 0, files.len(), "Proses başlanır...", &format!("{} fayl işlənəcək", files.len()));
    
    let mut results = Vec::new();
    
    // Process each file with corresponding Excel name
    for (index, file_name) in files.iter().enumerate() {
        // Check if process should stop
        if state.should_stop() {
            emit_progress(&window, index, files.len(), "Dayandırıldı", "Proses dayandırıldı");
            state.reset();
            return Ok(results);
        }
        
        // Handle pause
        while state.is_paused() && !state.should_stop() {
            emit_progress(&window, index, files.len(), "Fasilə verildi", "Proses fasilədədir");
            sleep(Duration::from_millis(100)).await;
        }
        
        // Check again after pause
        if state.should_stop() {
            emit_progress(&window, index, files.len(), "Dayandırıldı", "Proses dayandırıldı");
            state.reset();
            return Ok(results);
        }
        
        let current = index + 1;
        state.current_index.store(current, Ordering::Relaxed);
        
        emit_progress(&window, current, files.len(), &format!("İşlənən fayl: {}", file_name), &format!("{}/{} fayl", current, files.len()));
        
        let old_file_path = source_dir.join(file_name);
        
        if !old_file_path.exists() {
            let error_msg = format!("❌ Xəta: '{}' faylı tapılmadı", file_name);
            results.push(error_msg.clone());
            
            emit_process_result(&window, false, &error_msg, file_name, "");
            continue;
        }
        
        // Get new name from Excel
        let new_name = if index < excel_names.len() {
            &excel_names[index]
        } else {
            let error_msg = format!("❌ Xəta: '{}' faylı üçün Excel adı yoxdur (sətir {})", file_name, start_row + index as u32);
            results.push(error_msg.clone());
            
            emit_process_result(&window, false, &error_msg, file_name, "");
            continue;
        };
        
        // Get file extension
        let extension = old_file_path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| format!(".{}", ext))
            .unwrap_or_default();
        
        // Create safe filename with extension
        let safe_new_name = format!("{}{}", sanitize_filename(new_name), extension);
        let new_file_path = dest_dir.join(&safe_new_name);
        
        // Add delay to show progress
        sleep(Duration::from_millis(500)).await;
        
        // Move and rename file
        match move_file(&old_file_path, &new_file_path) {
            Ok(_) => {
                let success_msg = format!("✅ Uğur: '{}' → '{}'", file_name, safe_new_name);
                results.push(success_msg.clone());
                
                emit_process_result(&window, true, &success_msg, file_name, &safe_new_name);
            }
            Err(e) => {
                let error_msg = format!("❌ Xəta: '{}' köçürülə bilmədi: {}", file_name, e);
                results.push(error_msg.clone());
                
                emit_process_result(&window, false, &error_msg, file_name, &safe_new_name);
            }
        }
    }
    
    // Send completion
    if !state.should_stop() {
        emit_progress(&window, files.len(), files.len(), "Tamamlandı!", "Bütün fayllar işləndi");
    }
    
    state.reset();
    Ok(results)
}

// ================================================================================================
// Excel Operations
// ================================================================================================

/// Reads a specific column from an Excel file
#[command]
pub async fn read_excel_column(
    excel_path: String,
    start_row: u32,
    column: String,
) -> Result<Vec<String>, String> {
    read_excel_names(&excel_path, start_row, &column)
}

// ================================================================================================
// Document Operations
// ================================================================================================

/// Creates a PDF from multiple files (placeholder implementation)
#[command]
pub async fn create_pdf(
    files: Vec<String>,
    output_path: String,
    title: String,
) -> Result<String, String> {
    // TODO: Implement PDF creation functionality
    // This would require a PDF library like `printpdf` or similar
    Ok(format!(
        "PDF yaradılması planlaşdırıldı: {} fayl -> {} (başlıq: {})",
        files.len(),
        output_path,
        title
    ))
}

// ================================================================================================
// Helper Functions
// ================================================================================================

/// Emits progress update to the frontend
fn emit_progress(window: &Window, current: usize, total: usize, step: &str, message: &str) {
    let percentage = if total > 0 { (current as f32 / total as f32) * 100.0 } else { 0.0 };
    
    let _ = window.emit("progress-update", ProgressUpdate {
        current,
        total,
        percentage,
        current_step: step.to_string(),
        message: message.to_string(),
    });
}

/// Emits process result to the frontend
fn emit_process_result(window: &Window, success: bool, message: &str, folder_name: &str, new_name: &str) {
    let _ = window.emit("process-result", ProcessResult {
        success,
        message: message.to_string(),
        folder_name: folder_name.to_string(),
        new_name: new_name.to_string(),
    });
}

/// Sanitizes filename by removing invalid characters
fn sanitize_filename(name: &str) -> String {
    let invalid_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    let mut result = name.to_string();
    
    for ch in invalid_chars.iter() {
        result = result.replace(*ch, "_");
    }
    
    // Remove leading/trailing whitespace and dots
    result = result.trim().trim_matches('.').to_string();
    
    // Ensure the name is not empty
    if result.is_empty() {
        result = "Adsız_Qovluq".to_string();
    }
    
    result
}

/// Windows-specific logical string comparison
#[cfg(windows)]
fn windows_logical_compare(a: &str, b: &str) -> std::cmp::Ordering {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    
    // Convert strings to wide strings (UTF-16) with null terminator
    let a_wide: Vec<u16> = OsStr::new(a).encode_wide().chain(std::iter::once(0)).collect();
    let b_wide: Vec<u16> = OsStr::new(b).encode_wide().chain(std::iter::once(0)).collect();
    
    // Call Windows API function for logical comparison
    let result = unsafe {
        StrCmpLogicalW(
            PCWSTR(a_wide.as_ptr()),
            PCWSTR(b_wide.as_ptr())
        )
    };
    
    match result {
        x if x < 0 => std::cmp::Ordering::Less,
        x if x > 0 => std::cmp::Ordering::Greater,
        _ => std::cmp::Ordering::Equal,
    }
}

/// Custom logical sort with Azerbaijani alphabet support
fn custom_logical_sort(a: &str, b: &str) -> std::cmp::Ordering {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    
    let mut i = 0;
    
    while i < a_chars.len() && i < b_chars.len() {
        let a_char = a_chars[i];
        let b_char = b_chars[i];
        
        // Check if both characters are digits
        if a_char.is_ascii_digit() && b_char.is_ascii_digit() {
            // Extract the full number
            let (a_num, a_end) = extract_number(&a_chars, i);
            let (b_num, b_end) = extract_number(&b_chars, i);
            
            // Compare numbers numerically
            match a_num.cmp(&b_num) {
                std::cmp::Ordering::Equal => {
                    i = a_end.max(b_end);
                    continue;
                }
                other => return other,
            }
        } else {
            // Compare using Azerbaijani alphabet order
            let a_order = get_azerbaijani_char_order(a_char.to_lowercase().next().unwrap_or(a_char));
            let b_order = get_azerbaijani_char_order(b_char.to_lowercase().next().unwrap_or(b_char));
            
            match a_order.cmp(&b_order) {
                std::cmp::Ordering::Equal => {
                    i += 1;
                    continue;
                }
                other => return other,
            }
        }
    }
    
    // If all compared characters are equal, compare by length
    a_chars.len().cmp(&b_chars.len())
}

/// Extracts a number from character array starting at given position
fn extract_number(chars: &[char], start: usize) -> (u64, usize) {
    let mut num_str = String::new();
    let mut i = start;
    
    while i < chars.len() && chars[i].is_ascii_digit() {
        num_str.push(chars[i]);
        i += 1;
    }
    
    let num = num_str.parse::<u64>().unwrap_or(0);
    (num, i)
}

/// Returns the order of a character in the Azerbaijani alphabet
fn get_azerbaijani_char_order(ch: char) -> u32 {
    match ch {
        'a' => 1, 'b' => 2, 'c' => 3, 'ç' => 4, 'd' => 5, 'e' => 6, 'ə' => 7, 'f' => 8,
        'g' => 9, 'ğ' => 10, 'h' => 11, 'x' => 12, 'ı' => 13, 'i' => 14, 'j' => 15, 'k' => 16,
        'q' => 17, 'l' => 18, 'm' => 19, 'n' => 20, 'o' => 21, 'ö' => 22, 'p' => 23, 'r' => 24,
        's' => 25, 'ş' => 26, 't' => 27, 'u' => 28, 'ü' => 29, 'v' => 30, 'w' => 31, 'y' => 32, 'z' => 33,
        _ => ch as u32 + 1000, // Non-Azerbaijani characters come after
    }
}

/// Natural sort comparison (like Windows Explorer)
fn natural_sort_compare(a: &str, b: &str) -> std::cmp::Ordering {
    #[cfg(windows)]
    {
        windows_logical_compare(a, b)
    }
    
    #[cfg(not(windows))]
    {
        custom_logical_sort(a, b)
    }
}

/// Calculates the total size of a folder
fn get_folder_size(path: &str) -> Result<u64, std::io::Error> {
    let mut total_size = 0;
    
    fn visit_dir(dir: &Path, total_size: &mut u64) -> Result<(), std::io::Error> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                visit_dir(&path, total_size)?;
            } else {
                *total_size += entry.metadata()?.len();
            }
        }
        Ok(())
    }
    
    visit_dir(Path::new(path), &mut total_size)?;
    Ok(total_size)
}

/// Reads names from Excel file at specified column and starting row
fn read_excel_names(excel_path: &str, start_row: u32, column: &str) -> Result<Vec<String>, String> {
    let mut workbook: Xlsx<_> = open_workbook(excel_path)
        .map_err(|e| format!("Excel faylını açmaq mümkün olmadı: {}", e))?;
    
    let worksheet_name = workbook.sheet_names().first()
        .ok_or("Excel faylında heç bir iş vərəqi tapılmadı")?
        .clone();
    
    let range = workbook.worksheet_range(&worksheet_name)
        .ok_or("İş vərəqinin sahəsini əldə etmək mümkün olmadı")?
        .map_err(|e| format!("İş vərəqini oxumaq mümkün olmadı: {}", e))?;
    
    let column_index = column_letter_to_index(column)?;
    let mut names = Vec::new();
    
    // Read from start_row (1-indexed) to end of data
    for row in (start_row - 1)..range.height() as u32 {
        if let Some(cell) = range.get((row as usize, column_index)) {
            match cell {
                DataType::String(s) => {
                    let trimmed = s.trim();
                    if !trimmed.is_empty() {
                        names.push(trimmed.to_string());
                    }
                }
                DataType::Float(f) => {
                    names.push(f.to_string());
                }
                DataType::Int(i) => {
                    names.push(i.to_string());
                }
                _ => {} // Skip other data types
            }
        }
    }
    
    Ok(names)
}

/// Converts column letter (A, B, C, etc.) to zero-based index
fn column_letter_to_index(column: &str) -> Result<usize, String> {
    let column = column.to_uppercase();
    let mut result = 0;
    
    for ch in column.chars() {
        if !ch.is_ascii_alphabetic() {
            return Err(format!("Yanlış sütun hərfi: {}", column));
        }
        result = result * 26 + (ch as usize - 'A' as usize + 1);
    }
    
    Ok(result - 1) // Convert to zero-based index
}

/// Moves a folder from source to destination
fn move_folder(source: &Path, destination: &Path) -> Result<(), String> {
    // Try direct rename first (fastest if on same filesystem)
    match fs::rename(source, destination) {
        Ok(_) => Ok(()),
        Err(_) => {
            // If rename fails, copy and delete
            copy_dir_recursive(source, destination)?;
            fs::remove_dir_all(source)
                .map_err(|e| format!("Əsas qovluğu silmək mümkün olmadı: {}", e))?;
            Ok(())
        }
    }
}

/// Recursively copies a directory
fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<(), String> {
    fs::create_dir_all(destination)
        .map_err(|e| format!("Təyinat qovluq yaratmaq mümkün olmadı: {}", e))?;
    
    for entry in fs::read_dir(source)
        .map_err(|e| format!("Əsas qovluğu oxumaq mümkün olmadı: {}", e))? {
        let entry = entry.map_err(|e| format!("Qovluq girişini oxumaq mümkün olmadı: {}", e))?;
        let source_path = entry.path();
        let dest_path = destination.join(entry.file_name());
        
        if source_path.is_dir() {
            copy_dir_recursive(&source_path, &dest_path)?;
        } else {
            fs::copy(&source_path, &dest_path)
                .map_err(|e| format!("Faylı kopyalamaq mümkün olmadı: {}", e))?;
        }
    }
    
    Ok(())
}

/// Moves a file from source to destination
fn move_file(source: &Path, destination: &Path) -> Result<(), String> {
    // Try direct rename first (fastest if on same filesystem)
    match fs::rename(source, destination) {
        Ok(_) => Ok(()),
        Err(_) => {
            // If rename fails, copy and delete
            copy_file(source, destination)?;
            fs::remove_file(source)
                .map_err(|e| format!("Faylı silmək mümkün olmadı: {}", e))?;
            Ok(())
        }
    }
}

/// Copies a file from source to destination
fn copy_file(source: &Path, destination: &Path) -> Result<(), String> {
    fs::copy(source, destination)
        .map_err(|e| format!("Faylı kopyalamaq mümkün olmadı: {}", e))?;
    Ok(())
}

// ================================================================================================
// PDF Helper Functions
// ================================================================================================

/// Processes a single folder for PDF creation
async fn process_folder_for_pdf(
    folder_path: &Path,
    subfolder_path: &Path,
    _subfolder_name: &str,
    delete_files: &[String],
) -> Result<usize, String> {
    // Find all image files
    let mut image_files = Vec::new();
    
    match fs::read_dir(subfolder_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(extension) = path.extension() {
                            let ext = extension.to_string_lossy().to_lowercase();
                            if is_image_extension(&ext) {
                                image_files.push(path);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => return Err(format!("Şəkil qovluğu oxunması xətası: {}", e)),
    }

    if image_files.is_empty() {
        return Err("Şəkil faylları tapılmadı".to_string());
    }

    // Sort image files naturally
    image_files.sort_by(|a, b| {
        let a_name = a.file_name().unwrap_or_default().to_string_lossy();
        let b_name = b.file_name().unwrap_or_default().to_string_lossy();
        natural_sort_compare(&a_name, &b_name)
    });

    let images_count = image_files.len();
    
    // Create PDF
    let folder_name = folder_path.file_name()
        .ok_or("Qovluq adı alınmadı")?
        .to_string_lossy();
    let pdf_name = format!("{}_picture.pdf", folder_name);
    let pdf_path = subfolder_path.join(&pdf_name);

    create_pdf_from_image_files(&image_files, &pdf_path)?;

    // Delete original image files
    for image_path in &image_files {
        if let Err(e) = fs::remove_file(image_path) {
            eprintln!("Şəkil faylı silinmədi {}: {}", image_path.display(), e);
        }
    }

    // Delete specified files
    for file_name in delete_files {
        let file_path = subfolder_path.join(file_name);
        if file_path.exists() {
            if let Err(e) = fs::remove_file(&file_path) {
                eprintln!("Fayl silinmədi {}: {}", file_name, e);
            }
        }
    }

    // Move PDF and other files to parent folder
    move_files_to_parent(folder_path, subfolder_path, &pdf_name)?;

    // Remove empty subfolder
    if let Err(e) = fs::remove_dir(subfolder_path) {
        eprintln!("Boş qovluq silinmədi {}: {}", subfolder_path.display(), e);
    }

    Ok(images_count)
}

/// Creates PDF from image files using printpdf
fn create_pdf_from_image_files(image_files: &[std::path::PathBuf], output_path: &Path) -> Result<(), String> {
    use printpdf::*;
    use std::io::BufWriter;
    
    let (doc, page1, layer1) = PdfDocument::new("Image PDF", Mm(210.0), Mm(297.0), "Layer 1");
    let mut current_layer = doc.get_page(page1).get_layer(layer1);
    let mut is_first_page = true;

    for image_path in image_files {
        // Load image
        let img = ::image::open(image_path)
            .map_err(|e| format!("Şəkil açılması xətası {}: {}", image_path.display(), e))?;

        // Convert to RGB if needed
        let img = img.to_rgb8();
        let (width, height) = img.dimensions();

        // Calculate dimensions for A4 page
        let page_width = Mm(210.0);
        let page_height = Mm(297.0);
        let margin = Mm(10.0);
        let available_width = page_width - (margin * 2.0);
        let available_height = page_height - (margin * 2.0);

        // Calculate scale to fit image on page
        let scale_x = available_width.0 / (width as f32 * 0.264583); // Convert pixels to mm
        let scale_y = available_height.0 / (height as f32 * 0.264583);
        let scale = scale_x.min(scale_y);

        let final_width = Mm(width as f32 * 0.264583 * scale);
        let final_height = Mm(height as f32 * 0.264583 * scale);

        // Center image on page
        let x = (page_width - final_width) / 2.0;
        let y = (page_height - final_height) / 2.0;

        // Add new page if not first
        if !is_first_page {
            let (page, layer) = doc.add_page(page_width, page_height, "Layer");
            current_layer = doc.get_page(page).get_layer(layer);
        }
        is_first_page = false;

        // Create image object
        let image_bytes = img.into_raw();
        let image = Image::from_dynamic_image(&::image::DynamicImage::ImageRgb8(
            ::image::ImageBuffer::from_raw(width, height, image_bytes)
                .ok_or("Şəkil buffer yaradılması xətası")?
        ));

        // Add image to PDF
        image.add_to_layer(current_layer.clone(), ImageTransform {
            translate_x: Some(x),
            translate_y: Some(y),
            scale_x: Some(scale),
            scale_y: Some(scale),
            rotate: None,
            dpi: Some(300.0),
        });
    }

    // Save PDF
    let file = std::fs::File::create(output_path)
        .map_err(|e| format!("PDF fayl yaradılması xətası: {}", e))?;
    let mut buf_writer = BufWriter::new(file);
    doc.save(&mut buf_writer)
        .map_err(|e| format!("PDF saxlama xətası: {}", e))?;

    Ok(())
}

/// Checks if a file extension is an image format
fn is_image_extension(ext: &str) -> bool {
    matches!(ext, "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "tif" | "webp")
}

/// Checks if a directory contains image files
fn has_image_files(dir_path: &Path) -> Result<bool, std::io::Error> {
    let entries = fs::read_dir(dir_path)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if is_image_extension(&ext) {
                    return Ok(true);
                }
            }
        }
    }
    
    Ok(false)
}

/// Moves all files from subfolder to parent folder
fn move_files_to_parent(parent_folder: &Path, subfolder: &Path, _pdf_name: &str) -> Result<(), String> {
    match fs::read_dir(subfolder) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let source_path = entry.path();
                    if source_path.is_file() {
                        let file_name = entry.file_name();
                        let dest_path = parent_folder.join(&file_name);
                        
                        if let Err(e) = fs::rename(&source_path, &dest_path) {
                            eprintln!("Fayl köçürülmədi {}: {}", file_name.to_string_lossy(), e);
                        }
                    }
                }
            }
        }
        Err(e) => return Err(format!("Qovluq oxunması xətası: {}", e)),
    }
    
    Ok(())
}

/// Removes empty directories recursively
fn remove_empty_directories(root: &Path) -> Result<(), String> {
    let entries = fs::read_dir(root)
        .map_err(|e| format!("Qovluq oxunması xətası: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Entry oxunması xətası: {}", e))?;
        let path = entry.path();
        
        if path.is_dir() {
            // Recursively process subdirectories
            if let Err(e) = remove_empty_directories(&path) {
                eprintln!("Alt qovluq təmizlənmədi {}: {}", path.display(), e);
            }
            
            // Try to remove if empty
            if is_directory_empty(&path).unwrap_or(false) {
                if let Err(e) = fs::remove_dir(&path) {
                    eprintln!("Boş qovluq silinmədi {}: {}", path.display(), e);
                }
            }
        }
    }
    
    Ok(())
}

/// Checks if a directory is empty
fn is_directory_empty(dir_path: &Path) -> Result<bool, std::io::Error> {
    let mut entries = fs::read_dir(dir_path)?;
    Ok(entries.next().is_none())
} 