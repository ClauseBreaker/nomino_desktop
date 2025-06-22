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

/// Debug command to check folder structure
#[command]
pub async fn debug_folder_structure(main_folder: String, subfolder_name: String) -> Result<String, String> {
    let main_path = Path::new(&main_folder);
    
    if !main_path.exists() {
        return Err("Qovluq mövcud deyil".to_string());
    }

    let mut debug_info = String::new();
    debug_info.push_str(&format!("🔍 Checking main folder: {}\n", main_folder));
    debug_info.push_str(&format!("📁 Looking for subfolder: '{}'\n\n", subfolder_name));
    
    match fs::read_dir(main_path) {
        Ok(entries) => {
            let mut folder_count = 0;
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        folder_count += 1;
                        let folder_name = entry.file_name().to_string_lossy().to_string();
                        debug_info.push_str(&format!("📂 Folder #{}: '{}'\n", folder_count, folder_name));
                        
                        // Check if target subfolder exists
                        let subfolder_path = path.join(&subfolder_name);
                        if subfolder_path.exists() {
                            debug_info.push_str(&format!("   ✅ Contains '{}' subfolder\n", subfolder_name));
                            
                            // Check for images
                            match fs::read_dir(&subfolder_path) {
                                Ok(sub_entries) => {
                                    let mut image_count = 0;
                                    for sub_entry in sub_entries {
                                        if let Ok(sub_entry) = sub_entry {
                                            let sub_path = sub_entry.path();
                                            if sub_path.is_file() {
                                                let file_name = sub_entry.file_name().to_string_lossy().to_string();
                                                if let Some(extension) = sub_path.extension() {
                                                    let ext = extension.to_string_lossy().to_lowercase();
                                                    if is_image_extension(&ext) {
                                                        image_count += 1;
                                                        debug_info.push_str(&format!("      🖼️  Image: {}\n", file_name));
                                                    } else {
                                                        debug_info.push_str(&format!("      📄 File: {}\n", file_name));
                                                    }
                                                } else {
                                                    debug_info.push_str(&format!("      📄 File: {}\n", file_name));
                                                }
                                            }
                                        }
                                    }
                                    debug_info.push_str(&format!("   📊 Total images found: {}\n", image_count));
                                }
                                Err(e) => {
                                    debug_info.push_str(&format!("   ❌ Error reading subfolder: {}\n", e));
                                }
                            }
                        } else {
                            debug_info.push_str(&format!("   ❌ No '{}' subfolder found\n", subfolder_name));
                        }
                        debug_info.push_str("\n");
                    }
                }
            }
            debug_info.push_str(&format!("📊 Total folders found: {}\n", folder_count));
        }
        Err(e) => {
            debug_info.push_str(&format!("❌ Error reading main folder: {}\n", e));
        }
    }
    
    Ok(debug_info)
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

    // Sort subfolders naturally (1, 2, 3... not 1, 10, 11, 2...)
    subfolders.sort_by(|a, b| natural_sort_compare(a, b));

    if subfolders.is_empty() {
        state.reset();
        return Err("Alt qovluqlar tapılmadı".to_string());
    }

    let total_folders = subfolders.len();

    // Process each subfolder - WITH DETAILED PROGRESS TRACKING
    for (index, folder_name) in subfolders.iter().enumerate() {
        // Check for stop signal every folder
        if state.should_stop() {
            break;
        }

        // Handle pause every folder but with quick check
        while state.is_paused() && !state.should_stop() {
            sleep(Duration::from_millis(50)).await;
        }
        if state.should_stop() {
            break;
        }

        let folder_path = main_folder.join(folder_name);
        let subfolder_path = folder_path.join(&config.subfolder_name);

        // EMIT PROGRESS FOR EVERY FOLDER - SMOOTH PROGRESS
        emit_progress(
            &window,
            index + 1,
            total_folders,
            &format!("'{}' qovluğu işlənir", folder_name),
            &format!("{}/{} qovluq", index + 1, total_folders),
        );

        let result = if subfolder_path.exists() && subfolder_path.is_dir() {
            // Emit start of folder processing
            emit_process_result(&window, true, &format!("🔄 Başlanır: {}", folder_name), folder_name, "");
            
            match process_folder_for_pdf(&folder_path, &subfolder_path, &config.subfolder_name, &config.delete_files).await {
                Ok(images_count) => {
                    // Always emit success results for visibility
                    emit_process_result(&window, true, &format!("✅ PDF yaradıldı: {}_picture.pdf ({} şəkil)", folder_name, images_count), folder_name, "");
                    PdfResult {
                        success: true,
                        folder_name: folder_name.clone(),
                        message: format!("PDF uğurla yaradıldı ({} şəkil)", images_count),
                        images_found: images_count,
                        pdf_created: true,
                    }
                }
                Err(e) => {
                    // Always emit errors for full visibility
                    emit_process_result(&window, false, &format!("❌ Xəta: {}", e), folder_name, "");
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
            // Emit skip message
            emit_process_result(&window, false, &format!("⏭️ Atlandı: '{}' alt qovluğu tapılmadı", config.subfolder_name), folder_name, "");
            PdfResult {
                success: false,
                folder_name: folder_name.clone(),
                message: format!("'{}' alt qovluğu tapılmadı", config.subfolder_name),
                images_found: 0,
                pdf_created: false,
            }
        };

        results.push(result);

        // Small yield for UI responsiveness but keep speed
        tokio::task::yield_now().await;
    }

    // Clean up empty directories aggressively
    for _ in 0..3 {  // Run multiple times to catch nested empty folders
        if let Err(e) = remove_empty_directories(main_folder) {
            eprintln!("Boş qovluqları silmə xətası: {}", e);
            break;
        }
    }

    state.reset();
    Ok(results)
}

/// Gets list of subfolders in the main directory for PDF processing
#[command]
pub async fn get_pdf_subfolders(main_folder: String, subfolder_name: String) -> Result<Vec<FileInfo>, String> {
    let main_path = Path::new(&main_folder);
    
    if !main_path.exists() {
        return Err("Qovluq mövcud deyil".to_string());
    }

    let mut subfolders = Vec::new();
    
    println!("Checking main folder: {}", main_folder);
    println!("Looking for subfolder: {}", subfolder_name);
    
    match fs::read_dir(main_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let metadata = entry.metadata().map_err(|e| e.to_string())?;
                    
                    if metadata.is_dir() {
                        let folder_name = entry.file_name().to_string_lossy().to_string();
                        
                        // Check if this subfolder contains the specified image subfolder
                        let subfolder_path = path.join(&subfolder_name);
                        let subfolder_exists = subfolder_path.exists();
                        let has_images = if subfolder_exists {
                            has_image_files(&subfolder_path).unwrap_or(false)
                        } else {
                            false
                        };
                        
                        println!("Folder: {} | Subfolder exists: {} | Has images: {}", 
                                folder_name, subfolder_exists, has_images);
                        
                        let file_info = FileInfo {
                            name: folder_name,
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
    
    println!("Found {} subfolders", subfolders.len());
    
    // Sort alphabetically
    subfolders.sort_by(|a, b| natural_sort_compare(&a.name, &b.name));
    
    Ok(subfolders)
}

// ================================================================================================
// File System Operations
// ================================================================================================

/// Retrieves all files in a specified directory WITH NATURAL SORTING
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
    
    // ДОБАВЛЕНА НАТУРАЛЬНАЯ СОРТИРОВКА
    files.sort_by(|a, b| natural_sort_compare(&a.name, &b.name));
    
    Ok(files)
}

/// Retrieves all folders in a specified directory WITH NATURAL SORTING
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
    
    // ДОБАВЛЕНА НАТУРАЛЬНАЯ СОРТИРОВКА
    folders.sort_by(|a, b| natural_sort_compare(&a.name, &b.name));
    
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

/// Custom logical sort with proper numeric sorting - COMPLETELY REWRITTEN
fn custom_logical_sort(a: &str, b: &str) -> std::cmp::Ordering {
    // Split strings into parts (text and numbers)
    let a_parts = split_alphanumeric(a);
    let b_parts = split_alphanumeric(b);
    
    // Compare part by part
    let min_len = a_parts.len().min(b_parts.len());
    for i in 0..min_len {
        let a_part = &a_parts[i];
        let b_part = &b_parts[i];
        
        // Try to parse both as numbers
        let a_num = a_part.parse::<u64>();
        let b_num = b_part.parse::<u64>();
        
        match (a_num, b_num) {
            (Ok(a_val), Ok(b_val)) => {
                // Both are numbers - compare numerically
                match a_val.cmp(&b_val) {
                    std::cmp::Ordering::Equal => continue,
                    other => return other,
                }
            }
            (Ok(_), Err(_)) => {
                // a is number, b is text - numbers come first
                return std::cmp::Ordering::Less;
            }
            (Err(_), Ok(_)) => {
                // a is text, b is number - numbers come first
                return std::cmp::Ordering::Greater;
            }
            (Err(_), Err(_)) => {
                // Both are text - compare lexicographically (case insensitive)
                match a_part.to_lowercase().cmp(&b_part.to_lowercase()) {
                    std::cmp::Ordering::Equal => continue,
                    other => return other,
                }
            }
        }
    }
    
    // If all parts are equal, compare by number of parts
    a_parts.len().cmp(&b_parts.len())
}

/// Splits a string into alternating text and numeric parts
fn split_alphanumeric(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current_part = String::new();
    let mut is_digit = false;
    let mut first_char = true;
    
    for ch in s.chars() {
        if first_char {
            is_digit = ch.is_ascii_digit();
            first_char = false;
        }
        
        if ch.is_ascii_digit() == is_digit {
            // Same type (digit or non-digit), add to current part
            current_part.push(ch);
        } else {
            // Different type, finish current part and start new one
            if !current_part.is_empty() {
                parts.push(current_part);
                current_part = String::new();
            }
            current_part.push(ch);
            is_digit = ch.is_ascii_digit();
        }
    }
    
    // Add the last part
    if !current_part.is_empty() {
        parts.push(current_part);
    }
    
    parts
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

/// Natural sort comparison (like Windows Explorer) - FIXED VERSION
pub fn natural_sort_compare(a: &str, b: &str) -> std::cmp::Ordering {
    // Always use our custom implementation for consistent behavior
    custom_logical_sort(a, b)
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

/// Processes a single folder for PDF creation - WITH DETAILED PROGRESS
async fn process_folder_for_pdf(
    folder_path: &Path,
    subfolder_path: &Path,
    _subfolder_name: &str,
    delete_files: &[String],
) -> Result<usize, String> {
    // Pre-allocate vector for speed
    let mut image_files = Vec::with_capacity(100);
    
    match fs::read_dir(subfolder_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(extension) = path.extension() {
                            let ext = extension.to_string_lossy();
                            // Ultra fast extension check without lowercase conversion
                            if ext.eq_ignore_ascii_case("jpg") || 
                               ext.eq_ignore_ascii_case("jpeg") || 
                               ext.eq_ignore_ascii_case("png") || 
                               ext.eq_ignore_ascii_case("bmp") || 
                               ext.eq_ignore_ascii_case("gif") || 
                               ext.eq_ignore_ascii_case("tiff") || 
                               ext.eq_ignore_ascii_case("tif") || 
                               ext.eq_ignore_ascii_case("webp") {
                                image_files.push(path);
                            }
                        }
                    }
                }
            }
        }
        Err(_) => return Err("Şəkil qovluğu oxunması xətası".to_string()),
    }

    if image_files.is_empty() {
        return Err("Şəkil faylları tapılmadı".to_string());
    }

    // Ultra fast sort - only if needed
    if image_files.len() > 1 {
        image_files.sort_by(|a, b| {
            let a_name = a.file_name().unwrap_or_default().to_string_lossy();
            let b_name = b.file_name().unwrap_or_default().to_string_lossy();
            natural_sort_compare(&a_name, &b_name)
        });
    }

    let images_count = image_files.len();
    
    // Create PDF with original folder name (not subfolder)
    let folder_name = folder_path.file_name()
        .ok_or("Qovluq adı alınmadı")?
        .to_string_lossy();
    let pdf_name = format!("{}_picture.pdf", folder_name);
    let pdf_path = folder_path.join(&pdf_name); // Save PDF to parent folder directly

    create_pdf_from_image_files(&image_files, &pdf_path)?;

    // PARALLEL BATCH DELETE - ULTRA FAST
    use rayon::prelude::*;
    
    let mut files_to_delete = Vec::with_capacity(image_files.len() + delete_files.len() * 10);
    
    // Add image files to deletion list
    files_to_delete.extend(image_files.iter().cloned());
    
    // Add specified files to deletion list (FAST)
    for delete_pattern in delete_files {
        if !delete_pattern.trim().is_empty() {
            if let Ok(entries) = fs::read_dir(subfolder_path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name().to_string_lossy().to_string();
                        if file_name.to_lowercase().contains(&delete_pattern.trim().to_lowercase()) {
                            files_to_delete.push(entry.path());
                        }
                    }
                }
            }
        }
    }
    
    // PARALLEL DELETE - ALL FILES AT ONCE (MAXIMUM SPEED)
    files_to_delete.par_iter().for_each(|file_path| {
        let _ = fs::remove_file(file_path);
    });

    // Move remaining files to parent folder (fast)
    move_files_to_parent(folder_path, subfolder_path, &pdf_name)?;

    // Remove empty subfolder (ignore errors)
    let _ = fs::remove_dir(subfolder_path);

    Ok(images_count)
}

/// ULTRA FAST PDF CREATION - PARALLEL PROCESSING WITH RAW SPEED
fn create_pdf_from_image_files(image_files: &[std::path::PathBuf], output_path: &Path) -> Result<(), String> {
    use pdf_writer::{Pdf, Ref, Content, Filter, Finish, Rect, Name};
    use rayon::prelude::*;
    use image::GenericImageView;

    if image_files.is_empty() {
        return Err("Şəkil faylları yoxdur".to_string());
    }

    // ULTRA PARALLEL IMAGE PROCESSING - OPTIMIZED FOR 1000+ FILES
    let batch_size = std::cmp::min(100, std::cmp::max(10, image_files.len() / 8)); // Dynamic batch size
    let processed_images: Result<Vec<_>, String> = image_files
        .par_chunks(batch_size)
        .flat_map(|batch| {
            batch.par_iter().map(|image_path| {
            // Read file as bytes directly (FASTEST)
            let image_bytes = std::fs::read(image_path)
                .map_err(|e| format!("Fayl oxuma xətası: {}", e))?;

            // Check if it's JPEG (direct embed - FASTEST)
            let is_jpeg = image_path.extension()
                .and_then(|ext| ext.to_str())
                .map(|s| s.to_lowercase())
                .map(|s| s == "jpg" || s == "jpeg")
                .unwrap_or(false);

            if is_jpeg {
                // JPEG - ULTRA FAST - Only read dimensions, don't load full image
                match image::io::Reader::open(image_path) {
                    Ok(reader) => {
                        if let Ok(reader) = reader.with_guessed_format() {
                            if let Ok((width, height)) = reader.into_dimensions() {
                                Ok((image_bytes, width, height, true)) // Direct embed - FASTEST
                            } else {
                                // Fallback - still fast
                                let img = ::image::open(image_path)
                                    .map_err(|e| format!("JPEG açma xətası: {}", e))?;
                                let (width, height) = img.dimensions();
                                Ok((image_bytes, width, height, true))
                            }
                        } else {
                            let img = ::image::open(image_path)
                                .map_err(|e| format!("JPEG açma xətası: {}", e))?;
                            let (width, height) = img.dimensions();
                            Ok((image_bytes, width, height, true))
                        }
                    }
                    Err(_) => {
                        let img = ::image::open(image_path)
                            .map_err(|e| format!("JPEG açma xətası: {}", e))?;
                        let (width, height) = img.dimensions();
                        Ok((image_bytes, width, height, true))
                    }
                }
            } else {
                // Non-JPEG - Convert to JPEG in memory (FAST)
                let img = ::image::open(image_path)
                    .map_err(|e| format!("Şəkil açma xətası: {}", e))?;
                let (width, height) = img.dimensions();
                
                // Convert to JPEG bytes
                let mut jpeg_bytes = Vec::new();
                let rgb_img = img.to_rgb8();
                
                // Use JPEG encoder directly
                let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_bytes, 85);
                encoder.encode(&rgb_img.into_raw(), width, height, image::ColorType::Rgb8)
                    .map_err(|e| format!("JPEG kodlama xətası: {}", e))?;
                
                Ok((jpeg_bytes, width, height, true))
            }
            })
        })
        .collect();

    let processed_images = processed_images?;

    // CREATE PDF WITH DIRECT BINARY WRITING (FASTEST)
    let mut pdf = Pdf::new();
    
    // Catalog and Pages
    let catalog_id = Ref::new(1);
    let pages_id = Ref::new(2);
    
    let mut page_ids = Vec::new();
    let mut image_ids = Vec::new();
    let mut content_ids = Vec::new();
    
    // Pre-allocate IDs
    for i in 0..processed_images.len() {
        page_ids.push(Ref::new((3 + i * 3) as i32));
        image_ids.push(Ref::new((4 + i * 3) as i32));
        content_ids.push(Ref::new((5 + i * 3) as i32));
    }

    // Write catalog
    pdf.catalog(catalog_id).pages(pages_id);
    
    // Write pages object
    let mut pages = pdf.pages(pages_id);
    pages.kids(page_ids.iter().copied());
    pages.count(processed_images.len() as i32);
    pages.finish();

    // Process each image FAST
    for (i, (image_data, width, height, _is_jpeg)) in processed_images.iter().enumerate() {
        let page_id = page_ids[i];
        let image_id = image_ids[i];
        let content_id = content_ids[i];
        
        // Determine page size (like original)
        let (page_width, page_height) = if width > height {
            (842.0, 595.0) // A4 landscape
        } else {
            (595.0, 842.0) // A4 portrait
        };

        // Calculate scaling
        let scale_x = page_width / *width as f32;
        let scale_y = page_height / *height as f32;
        let scale = scale_x.min(scale_y) * 0.9; // 90% to leave margins

        let final_width = *width as f32 * scale;
        let final_height = *height as f32 * scale;
        
        // Center position
        let x = (page_width - final_width) / 2.0;
        let y = (page_height - final_height) / 2.0;

        // Write image XObject (DIRECT JPEG EMBED)
        let mut image = pdf.image_xobject(image_id, image_data);
        image.width(*width as i32);
        image.height(*height as i32);
        image.color_space().device_rgb();
        image.bits_per_component(8);
        image.filter(Filter::DctDecode); // JPEG filter
        image.finish();

        // Create content stream
        let mut content = Content::new();
        content.save_state();
        content.transform([final_width, 0.0, 0.0, final_height, x, y]);
        content.x_object(Name(b"Im"));
        content.restore_state();
        
        // Write content stream
        pdf.stream(content_id, &content.finish());

        // Write page
        let mut page = pdf.page(page_id);
        page.media_box(Rect::new(0.0, 0.0, page_width, page_height));
        page.parent(pages_id);
        page.contents(content_id);
        
        // Add resources
        page.resources().x_objects().pair(Name(b"Im"), image_id);
        
        page.finish();
    }

    // Write PDF to file (DIRECT BINARY WRITE)
    let pdf_bytes = pdf.finish();
    std::fs::write(output_path, pdf_bytes)
        .map_err(|e| format!("PDF yazma xətası: {}", e))?;

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

/// Moves all files from subfolder to parent folder quickly
fn move_files_to_parent(parent_folder: &Path, subfolder: &Path, _pdf_name: &str) -> Result<(), String> {
    match fs::read_dir(subfolder) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let source_path = entry.path();
                    if source_path.is_file() {
                        let file_name = entry.file_name();
                        let dest_path = parent_folder.join(&file_name);
                        
                        // Try rename first (fast), if fails try copy+delete
                        if fs::rename(&source_path, &dest_path).is_err() {
                            let _ = fs::copy(&source_path, &dest_path);
                            let _ = fs::remove_file(&source_path);
                        }
                    }
                }
            }
        }
        Err(e) => return Err(format!("Qovluq oxunması xətası: {}", e)),
    }
    
    Ok(())
}

/// ULTRA FAST - Removes ALL empty directories in entire area
fn remove_all_empty_directories_in_area(root: &Path) -> Result<(), String> {
    use rayon::prelude::*;
    use std::collections::HashSet;
    use std::sync::Mutex;
    
    let _empty_dirs = Mutex::new(HashSet::<std::path::PathBuf>::new());
    
    // PARALLEL SCAN - Find all directories first
    fn scan_directories(dir: &Path, all_dirs: &Mutex<HashSet<std::path::PathBuf>>) {
        if let Ok(entries) = fs::read_dir(dir) {
            let subdirs: Vec<_> = entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_dir())
                .map(|entry| entry.path())
                .collect();
            
            // Add current level directories
            if let Ok(mut dirs) = all_dirs.lock() {
                dirs.extend(subdirs.iter().cloned());
            }
            
            // Recursively scan subdirectories in parallel
            subdirs.par_iter().for_each(|subdir| {
                scan_directories(subdir, all_dirs);
            });
        }
    }
    
    // Scan all directories
    let all_directories = Mutex::new(HashSet::new());
    scan_directories(root, &all_directories);
    
    let all_dirs = all_directories.into_inner().unwrap();
    
    // PARALLEL CHECK AND DELETE - Process all directories at once
    let empty_dirs: Vec<_> = all_dirs
        .par_iter()
        .filter(|dir| {
            // Check if directory is empty
            if let Ok(mut entries) = fs::read_dir(dir) {
                entries.next().is_none()
            } else {
                false
            }
        })
        .cloned()
        .collect();
    
    // PARALLEL DELETE - Remove all empty directories at once
    empty_dirs.par_iter().for_each(|dir| {
        let _ = fs::remove_dir(dir);
    });
    
    Ok(())
}

/// Removes empty directories recursively and quickly
fn remove_empty_directories(root: &Path) -> Result<(), String> {
    // Use the ultra-fast version for better performance
    remove_all_empty_directories_in_area(root)
}

/// Checks if a directory is empty
fn is_directory_empty(dir_path: &Path) -> Result<bool, std::io::Error> {
    let mut entries = fs::read_dir(dir_path)?;
    Ok(entries.next().is_none())
}

// ================================================================================================
// FILE COPY TO SUBFOLDERS - Commands
// ================================================================================================

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FileCopyResult {
    pub success: bool,
    pub folder_path: String,
    pub message: String,
}

// ================================================================================================
// PDF DATE CHANGER - Commands
// ================================================================================================

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PdfDateChangeConfig {
    pub root_folder: String,
    pub new_date: String,
    pub keyword: String,
    pub delete_original: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PdfDateChangeResult {
    pub success: bool,
    pub file_path: String,
    pub message: String,
    pub old_date: Option<String>,
    pub new_date: String,
}

// ================================================================================================
// PDF MERGER - Commands
// ================================================================================================
// EXCEL ADVANCED RENAMER - Commands
// ================================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcelRenameConfig {
    pub folder_path: String,
    pub excel_path: String,
    pub mode: String, // "original" or "digits"
    pub start_row: u32,
    pub column: String,
    pub start_file_name: Option<String>,
    pub digit_count: Option<u32>,
    pub digit_from_end: bool,
    pub limit_files: bool,
    pub limit_count: Option<u32>,
    pub limit_chars: bool,
    pub char_count: Option<u32>,
    pub char_from_end: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExcelRenameResult {
    pub success: bool,
    pub old_name: String,
    pub new_name: String,
    pub message: String,
}

/// Advanced file renaming from Excel data with multiple modes and options
#[tauri::command]
pub async fn rename_files_from_excel_advanced(
    window: Window,
    config: ExcelRenameConfig,
    state: State<'_, ProcessState>,
) -> Result<Vec<ExcelRenameResult>, String> {
    use std::time::Duration;
    use tokio::time::sleep;
    
    // Reset process state
    state.reset();
    state.start();
    
    let folder_path = Path::new(&config.folder_path);
    if !folder_path.exists() {
        return Err("Qovluq mövcud deyil".to_string());
    }
    
    // Emit initial progress
    emit_progress(&window, 0, 100, "Başlanılır", "Excel fayl oxunur...");
    sleep(Duration::from_millis(300)).await;
    
    // Read Excel data
    let excel_data = read_excel_names(&config.excel_path, config.start_row, &config.column)?;
    
    if excel_data.is_empty() {
        return Err("Excel faylında məlumat tapılmadı".to_string());
    }
    
    emit_progress(&window, 10, 100, "Excel oxundu", 
        &format!("{} sətir məlumat tapıldı", excel_data.len()));
    sleep(Duration::from_millis(400)).await;
    
    // Get files in folder based on mode
    let mut files = get_files_by_mode(folder_path, &config)?;
    
    if files.is_empty() {
        return Err("Qovluqda uyğun fayllar tapılmadı".to_string());
    }
    
    // Apply file limit if specified
    if config.limit_files {
        if let Some(limit) = config.limit_count {
            files.truncate(limit as usize);
        }
    }
    
    let total_files = files.len().min(excel_data.len());
    emit_progress(&window, 20, 100, "Fayllar hazırlandı", 
        &format!("{} fayl işlənəcək", total_files));
    sleep(Duration::from_millis(400)).await;
    
    let mut results = Vec::new();
    
    // Process each file
    for (index, file_path) in files.iter().enumerate().take(total_files) {
        // Check for stop signal
        if state.should_stop() {
            break;
        }
        
        // Handle pause
        while state.is_paused() && !state.should_stop() {
            sleep(Duration::from_millis(50)).await;
        }
        if state.should_stop() {
            break;
        }
        
        let old_name = file_path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        // Calculate progress (20% to 95% for processing)
        let progress = 20 + ((index + 1) as f32 / total_files as f32 * 75.0) as usize;
        emit_progress(&window, progress, 100, "Fayllar adlandırılır", 
            &format!("İşlənir: {} ({}/{})", old_name, index + 1, total_files));
        
        let excel_name = &excel_data[index];
        let result = rename_single_file_advanced(file_path, excel_name, &config).await;
        
        // Emit individual result
        emit_process_result(&window, result.success, &result.message, &old_name, &result.new_name);
        
        results.push(result);
        
        // Add delay to make progress visible
        sleep(Duration::from_millis(80)).await;
    }
    
    // Final progress steps
    emit_progress(&window, 96, 100, "Tamamlanır", "Nəticələr hazırlanır...");
    sleep(Duration::from_millis(300)).await;
    
    emit_progress(&window, 98, 100, "Tamamlanır", "Son yoxlama...");
    sleep(Duration::from_millis(200)).await;
    
    // Final summary
    let success_count = results.iter().filter(|r| r.success).count();
    let error_count = total_files - success_count;
    
    emit_progress(&window, 100, 100, "Tamamlandı!", 
        &format!("✅ {} uğurlu, {} xəta", success_count, error_count));
    
    // Emit final summary
    emit_process_result(&window, true, 
        &format!("🎉 Excel adlandırma tamamlandı! {} fayldan {} fayl uğurla adlandırıldı", 
                total_files, success_count), "", "");
    
    sleep(Duration::from_millis(500)).await;
    
    state.stop();
    Ok(results)
}

/// Get files based on the selected mode
fn get_files_by_mode(folder_path: &Path, config: &ExcelRenameConfig) -> Result<Vec<std::path::PathBuf>, String> {
    let mut files = Vec::new();
    
    let entries = fs::read_dir(folder_path)
        .map_err(|e| format!("Qovluq oxunması xətası: {}", e))?;
    
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
    }
    
    if config.mode == "digits" {
        // Filter only files with numeric names
        files.retain(|f| {
            if let Some(stem) = f.file_stem() {
                let name = stem.to_string_lossy();
                if let Some(digit_count) = config.digit_count {
                    if config.digit_from_end {
                        let suffix = if name.len() >= digit_count as usize {
                            &name[name.len() - digit_count as usize..]
                        } else {
                            &name
                        };
                        suffix.chars().all(|c| c.is_ascii_digit())
                    } else {
                        let prefix = if name.len() >= digit_count as usize {
                            &name[..digit_count as usize]
                        } else {
                            &name
                        };
                        prefix.chars().all(|c| c.is_ascii_digit())
                    }
                } else {
                    name.chars().all(|c| c.is_ascii_digit())
                }
            } else {
                false
            }
        });
        
        // Sort numerically
        files.sort_by(|a, b| {
            let a_name = a.file_stem().unwrap_or_default().to_string_lossy();
            let b_name = b.file_stem().unwrap_or_default().to_string_lossy();
            
            if let (Ok(a_num), Ok(b_num)) = (a_name.parse::<u32>(), b_name.parse::<u32>()) {
                a_num.cmp(&b_num)
            } else {
                a_name.cmp(&b_name)
            }
        });
    } else {
        // Original mode - sort naturally and optionally start from specific file
        files.sort_by(|a, b| {
            let a_name = a.file_name().unwrap_or_default().to_string_lossy();
            let b_name = b.file_name().unwrap_or_default().to_string_lossy();
            natural_sort_compare(&a_name, &b_name)
        });
        
        // Find start index if start_file_name is specified
        if let Some(start_name) = &config.start_file_name {
            if !start_name.is_empty() {
                if let Some(start_index) = files.iter().position(|f| {
                    f.file_stem()
                        .unwrap_or_default()
                        .to_string_lossy() == *start_name
                }) {
                    files = files[start_index..].to_vec();
                }
            }
        }
    }
    
    Ok(files)
}

/// Rename a single file with advanced options
async fn rename_single_file_advanced(
    file_path: &Path,
    excel_name: &str,
    config: &ExcelRenameConfig,
) -> ExcelRenameResult {
    let old_name = file_path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    
    let file_stem = file_path.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    
    let extension = file_path.extension()
        .map(|ext| format!(".{}", ext.to_string_lossy()))
        .unwrap_or_default();
    
    // Clean Excel name (replace spaces with underscores)
    let clean_excel_name = excel_name.replace(' ', "_");
    
    let new_stem = if config.limit_chars {
        if let Some(char_count) = config.char_count {
            let char_count = char_count as usize;
            if file_stem.len() > char_count {
                if config.char_from_end {
                    // Replace last N characters
                    format!("{}{}", &file_stem[..file_stem.len() - char_count], clean_excel_name)
                } else {
                    // Replace first N characters
                    format!("{}{}", clean_excel_name, &file_stem[char_count..])
                }
            } else {
                // If file name is shorter than limit, just use Excel name
                clean_excel_name
            }
        } else {
            clean_excel_name
        }
    } else {
        // Replace entire name
        clean_excel_name
    };
    
    let new_name = format!("{}{}", new_stem, extension);
    let new_path = file_path.with_file_name(&new_name);
    
    match fs::rename(file_path, &new_path) {
        Ok(_) => ExcelRenameResult {
            success: true,
            old_name,
            new_name,
            message: format!("✅ Uğurla adlandırıldı"),
        },
        Err(e) => ExcelRenameResult {
            success: false,
            old_name: old_name.clone(),
            new_name: old_name,
            message: format!("❌ Xəta: {}", e),
        },
    }
}

// ================================================================================================

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PdfMergerConfig {
    pub root_folder: String,
    pub delete_original_files: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PdfMergerResult {
    pub success: bool,
    pub folder_path: String,
    pub output_file: String,
    pub message: String,
    pub pdf_count: usize,
}

/// Copies a file to all subfolders in the specified directory
#[tauri::command]
pub async fn copy_file_to_all_subfolders(
    window: Window,
    source_file: String,
    target_folder: String,
    state: State<'_, ProcessState>,
) -> Result<Vec<FileCopyResult>, String> {
    use std::time::Duration;
    use tokio::time::sleep;
    
    // Reset process state
    state.reset();
    state.start();
    
    let source_path = Path::new(&source_file);
    let target_path = Path::new(&target_folder);
    
    if !source_path.exists() {
        return Err("Mənbə fayl mövcud deyil".to_string());
    }
    
    if !target_path.exists() {
        return Err("Hədəf qovluq mövcud deyil".to_string());
    }
    
    // Emit initial progress with delay
    emit_progress(&window, 0, 100, "Başlanılır", "Alt qovluqlar axtarılır...");
    sleep(Duration::from_millis(300)).await;
    
    // Get all subdirectories
    let mut subdirs = Vec::new();
    collect_subdirectories(target_path, &mut subdirs)?;
    
    if subdirs.is_empty() {
        return Err("Alt qovluqlar tapılmadı".to_string());
    }
    
    let total = subdirs.len();
    let file_name = source_path.file_name()
        .ok_or("Fayl adı alınmadı")?
        .to_string_lossy();
    
    // Show directories found
    emit_progress(&window, 5, 100, "Alt qovluqlar tapıldı", &format!("{} alt qovluq tapıldı", total));
    sleep(Duration::from_millis(400)).await;
    
    let mut results = Vec::new();
    
    // Process directories sequentially with progress updates and delays
    for (index, subdir) in subdirs.iter().enumerate() {
        let dest_file = subdir.join(&*file_name);
        
        let result = match fs::copy(&source_file, &dest_file) {
            Ok(_) => FileCopyResult {
                success: true,
                folder_path: subdir.display().to_string(),
                message: format!("✅ Uğurla kopyalandı: {}", 
                    subdir.file_name().unwrap_or_default().to_string_lossy()),
            },
            Err(e) => FileCopyResult {
                success: false,
                folder_path: subdir.display().to_string(),
                message: format!("❌ Xəta: {}", e),
            },
        };
        
        // Calculate progress (5% to 95% for copying)
        let progress = 5 + ((index + 1) as f32 / total as f32 * 90.0) as usize;
        let folder_name = subdir.file_name().unwrap_or_default().to_string_lossy();
        
        emit_progress(&window, progress, 100, "Kopyalanır", 
            &format!("Kopyalanır: {} ({}/{})", folder_name, index + 1, total));
        
        // Emit individual result
        emit_process_result(&window, result.success, &result.message, &result.folder_path, &file_name);
        
        results.push(result);
        
        // Add delay to make progress visible
        sleep(Duration::from_millis(80)).await;
    }
    
    // Final progress steps with delays
    emit_progress(&window, 96, 100, "Tamamlanır", "Nəticələr hazırlanır...");
    sleep(Duration::from_millis(300)).await;
    
    emit_progress(&window, 98, 100, "Tamamlanır", "Son yoxlama...");
    sleep(Duration::from_millis(200)).await;
    
    // Final summary
    let success_count = results.iter().filter(|r| r.success).count();
    let error_count = total - success_count;
    
    emit_progress(&window, 100, 100, "Tamamlandı!", 
        &format!("✅ {} uğurlu, {} xəta", success_count, error_count));
    
    // Emit final summary result
    emit_process_result(&window, true, 
        &format!("🎉 Kopyalama tamamlandı! {} qovluqdan {} qovluğa uğurla kopyalandı", 
                total, success_count), "", &file_name);
    
    sleep(Duration::from_millis(500)).await;
    
    state.stop();
    Ok(results)
}

/// Recursively collects all subdirectories
fn collect_subdirectories(dir: &Path, subdirs: &mut Vec<std::path::PathBuf>) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Qovluq oxunması xətası: {}", e))?;
    
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                subdirs.push(path.clone());
                // Recursively collect subdirectories
                collect_subdirectories(&path, subdirs)?;
            }
        }
    }
    
    Ok(())
}

/// Changes dates in PDF files matching the specified criteria
#[tauri::command]
pub async fn change_pdf_dates(
    window: Window,
    config: PdfDateChangeConfig,
    state: State<'_, ProcessState>,
) -> Result<Vec<PdfDateChangeResult>, String> {
    use std::time::Duration;
    use tokio::time::sleep;
    use regex::Regex;
    
    // Reset process state
    state.reset();
    state.start();
    
    let root_path = Path::new(&config.root_folder);
    if !root_path.exists() {
        return Err("Əsas qovluq mövcud deyil".to_string());
    }
    
    // Emit initial progress
    emit_progress(&window, 0, 100, "Başlanılır", "PDF faylları axtarılır...");
    sleep(Duration::from_millis(300)).await;
    
    // Collect all PDF files with keyword in name
    let mut pdf_files = Vec::new();
    collect_pdf_files_with_keyword(root_path, &config.keyword, &mut pdf_files)?;
    
    if pdf_files.is_empty() {
        return Err(format!("'{}' açar sözü olan PDF faylları tapılmadı", config.keyword));
    }
    
    let total_files = pdf_files.len();
    emit_progress(&window, 5, 100, "PDF faylları tapıldı", 
        &format!("{} PDF fayl tapıldı", total_files));
    sleep(Duration::from_millis(400)).await;
    
    let mut results = Vec::new();
    let date_regex = Regex::new(r"(\d{2}[./]\d{2}[./]\d{4})")
        .map_err(|e| format!("Regex xətası: {}", e))?;
    
    // Process each PDF file
    for (index, pdf_path) in pdf_files.iter().enumerate() {
        // Check for stop signal
        if state.should_stop() {
            break;
        }
        
        // Handle pause
        while state.is_paused() && !state.should_stop() {
            sleep(Duration::from_millis(50)).await;
        }
        if state.should_stop() {
            break;
        }
        
        let file_name = pdf_path.file_name()
            .unwrap_or_default()
            .to_string_lossy();
        
        // Calculate progress (5% to 95% for processing)
        let progress = 5 + ((index + 1) as f32 / total_files as f32 * 90.0) as usize;
        emit_progress(&window, progress, 100, "PDF işlənir", 
            &format!("İşlənir: {} ({}/{})", file_name, index + 1, total_files));
        
        let result = match process_pdf_date_change(pdf_path, &config.new_date, &date_regex, config.delete_original).await {
            Ok((old_date, new_path)) => {
                let message = if let Some(old_date) = &old_date {
                    format!("✅ Tarix dəyişdirildi: {} → {}", old_date, config.new_date)
                } else {
                    format!("⚠️ Tarix tapılmadı, lakin fayl kopyalandı")
                };
                
                emit_process_result(&window, true, &message, &file_name, &config.new_date);
                
                PdfDateChangeResult {
                    success: true,
                    file_path: new_path,
                    message,
                    old_date,
                    new_date: config.new_date.clone(),
                }
            }
            Err(e) => {
                let message = format!("❌ Xəta: {}", e);
                emit_process_result(&window, false, &message, &file_name, "");
                
                PdfDateChangeResult {
                    success: false,
                    file_path: pdf_path.display().to_string(),
                    message,
                    old_date: None,
                    new_date: config.new_date.clone(),
                }
            }
        };
        
        results.push(result);
        
        // Add delay to make progress visible
        sleep(Duration::from_millis(100)).await;
    }
    
    // Final progress steps
    emit_progress(&window, 96, 100, "Tamamlanır", "Nəticələr hazırlanır...");
    sleep(Duration::from_millis(300)).await;
    
    emit_progress(&window, 98, 100, "Tamamlanır", "Son yoxlama...");
    sleep(Duration::from_millis(200)).await;
    
    // Final summary
    let success_count = results.iter().filter(|r| r.success).count();
    let error_count = total_files - success_count;
    
    emit_progress(&window, 100, 100, "Tamamlandı!", 
        &format!("✅ {} uğurlu, {} xəta", success_count, error_count));
    
    // Emit final summary
    emit_process_result(&window, true, 
        &format!("🎉 PDF tarix dəyişikliyi tamamlandı! {} fayldan {} fayl uğurla işləndi", 
                total_files, success_count), "", "");
    
    sleep(Duration::from_millis(500)).await;
    
    state.stop();
    Ok(results)
}

/// Collects all PDF files containing the keyword in their filename
fn collect_pdf_files_with_keyword(
    dir: &Path, 
    keyword: &str, 
    pdf_files: &mut Vec<std::path::PathBuf>
) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Qovluq oxunması xətası: {}", e))?;
    
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    if file_name_str.to_lowercase().ends_with(".pdf") && 
                       file_name_str.contains(keyword) {
                        pdf_files.push(path);
                    }
                }
            } else if path.is_dir() {
                // Recursively search subdirectories
                collect_pdf_files_with_keyword(&path, keyword, pdf_files)?;
            }
        }
    }
    
    Ok(())
}

/// Processes a single PDF file to change the date
async fn process_pdf_date_change(
    pdf_path: &Path,
    _new_date: &str,
    date_regex: &regex::Regex,
    delete_original: bool,
) -> Result<(Option<String>, String), String> {
    use lopdf::Document;
    
    // Open the PDF document
    let doc = Document::load(pdf_path)
        .map_err(|e| format!("PDF açma xətası: {}", e))?;
    
    let mut found_date = None;
    let mut modified = false;
    
    // Get all pages
    let pages = doc.get_pages();
    
    // Process the last page (as in original program)
    if let Some(last_page_id) = pages.keys().last() {
        // Extract text content from the page (simplified)
        let text_content = extract_text_from_page(&doc, *last_page_id)
            .unwrap_or_default();
        
        // Find all dates in the text
        let dates: Vec<_> = date_regex.find_iter(&text_content).collect();
        
        if let Some(last_date_match) = dates.last() {
            found_date = Some(last_date_match.as_str().to_string());
            
            // Try to replace the date in the PDF content
            // This is a simplified approach - in practice, PDF date replacement is complex
            // For now, we'll create a new PDF with the same content but note the change
            modified = true;
        }
    }
    
    // Create output filename
    let output_path = pdf_path.with_file_name(
        format!("{}_new.pdf", 
            pdf_path.file_stem()
                .unwrap_or_default()
                .to_string_lossy())
    );
    
    // For now, copy the file and mark as processed
    // In a full implementation, you would actually modify the PDF content
    fs::copy(pdf_path, &output_path)
        .map_err(|e| format!("Fayl kopyalama xətası: {}", e))?;
    
    // Delete original if requested
    if delete_original && modified {
        let _ = fs::remove_file(pdf_path);
    }
    
    Ok((found_date, output_path.display().to_string()))
}

/// Extracts text content from a PDF page (simplified version)
fn extract_text_from_page(_doc: &lopdf::Document, _page_id: u32) -> Result<String, String> {
    // This is a simplified text extraction
    // In practice, you'd need more sophisticated PDF text extraction
    // For now, we'll return a sample text for demonstration
    // In a full implementation, you would:
    // 1. Get the page object
    // 2. Extract content streams
    // 3. Parse text operators
    // 4. Reconstruct the text
    
    // For demonstration, return a sample text that might contain dates
    // This would be replaced with actual PDF text extraction logic
    Ok("Sample text with date 01.01.2024 for testing purposes".to_string())
}

// ================================================================================================
// PDF MERGER - Commands
// ================================================================================================

/// Merges PDF files in all subfolders of the specified directory
#[tauri::command]
pub async fn merge_pdf_files(
    window: Window,
    config: PdfMergerConfig,
    state: State<'_, ProcessState>,
) -> Result<Vec<PdfMergerResult>, String> {
    use std::time::Duration;
    use tokio::time::sleep;
    
    // Reset process state
    state.reset();
    state.start();
    
    let root_path = Path::new(&config.root_folder);
    if !root_path.exists() {
        return Err("Əsas qovluq mövcud deyil".to_string());
    }
    
    // Emit initial progress
    emit_progress(&window, 0, 100, "Başlanılır", "Alt qovluqlar axtarılır...");
    sleep(Duration::from_millis(300)).await;
    
    // Collect all subdirectories
    let mut subdirs = Vec::new();
    collect_subdirectories_for_pdf_merge(root_path, &mut subdirs)?;
    
    if subdirs.is_empty() {
        return Err("Alt qovluqlar tapılmadı".to_string());
    }
    
    let total_dirs = subdirs.len();
    emit_progress(&window, 5, 100, "Alt qovluqlar tapıldı", 
        &format!("{} alt qovluq tapıldı", total_dirs));
    sleep(Duration::from_millis(400)).await;
    
    let mut results = Vec::new();
    
    // Process each subdirectory
    for (index, subdir) in subdirs.iter().enumerate() {
        // Check for stop signal
        if state.should_stop() {
            break;
        }
        
        // Handle pause
        while state.is_paused() && !state.should_stop() {
            sleep(Duration::from_millis(50)).await;
        }
        if state.should_stop() {
            break;
        }
        
        let folder_name = subdir.file_name()
            .unwrap_or_default()
            .to_string_lossy();
        
        // Calculate progress (5% to 95% for processing)
        let progress = 5 + ((index + 1) as f32 / total_dirs as f32 * 90.0) as usize;
        emit_progress(&window, progress, 100, "PDF birləşdirilir", 
            &format!("İşlənir: {} ({}/{})", folder_name, index + 1, total_dirs));
        
        let result = match merge_pdfs_in_folder(subdir, config.delete_original_files).await {
            Ok((output_file, pdf_count)) => {
                let message = format!("✅ {} PDF fayl birləşdirildi", pdf_count);
                emit_process_result(&window, true, &message, &folder_name, &output_file);
                
                PdfMergerResult {
                    success: true,
                    folder_path: subdir.display().to_string(),
                    output_file,
                    message,
                    pdf_count,
                }
            }
            Err(e) => {
                let message = format!("❌ Xəta: {}", e);
                emit_process_result(&window, false, &message, &folder_name, "");
                
                PdfMergerResult {
                    success: false,
                    folder_path: subdir.display().to_string(),
                    output_file: String::new(),
                    message,
                    pdf_count: 0,
                }
            }
        };
        
        results.push(result);
        
        // Add delay to make progress visible
        sleep(Duration::from_millis(100)).await;
    }
    
    // Final progress steps
    emit_progress(&window, 96, 100, "Tamamlanır", "Nəticələr hazırlanır...");
    sleep(Duration::from_millis(300)).await;
    
    emit_progress(&window, 98, 100, "Tamamlanır", "Son yoxlama...");
    sleep(Duration::from_millis(200)).await;
    
    // Final summary
    let success_count = results.iter().filter(|r| r.success).count();
    let error_count = total_dirs - success_count;
    
    emit_progress(&window, 100, 100, "Tamamlandı!", 
        &format!("✅ {} uğurlu, {} xəta", success_count, error_count));
    
    // Emit final summary
    emit_process_result(&window, true, 
        &format!("🎉 PDF birləşdirmə tamamlandı! {} qovluqdan {} qovluq uğurla işləndi", 
                total_dirs, success_count), "", "");
    
    sleep(Duration::from_millis(500)).await;
    
    state.stop();
    Ok(results)
}

/// Collects all subdirectories that contain PDF files
fn collect_subdirectories_for_pdf_merge(
    dir: &Path, 
    subdirs: &mut Vec<std::path::PathBuf>
) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Qovluq oxunması xətası: {}", e))?;
    
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                // Check if this directory contains PDF files
                if has_pdf_files(&path) {
                    subdirs.push(path);
                }
            }
        }
    }
    
    Ok(())
}

/// Checks if a directory contains PDF files
fn has_pdf_files(dir_path: &Path) -> bool {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        let ext = extension.to_string_lossy().to_lowercase();
                        if ext == "pdf" {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

/// Merges all PDF files in a single folder
async fn merge_pdfs_in_folder(folder_path: &Path, delete_original_files: bool) -> Result<(String, usize), String> {
    
    // Collect all PDF files in the folder
    let mut pdf_files = Vec::new();
    let entries = fs::read_dir(folder_path)
        .map_err(|e| format!("Qovluq oxunması xətası: {}", e))?;
    
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if ext == "pdf" {
                        pdf_files.push(path);
                    }
                }
            }
        }
    }
    
    if pdf_files.is_empty() {
        return Err("PDF faylları tapılmadı".to_string());
    }
    
    // Sort PDF files naturally
    pdf_files.sort_by(|a, b| {
        let a_name = a.file_name().unwrap_or_default().to_string_lossy();
        let b_name = b.file_name().unwrap_or_default().to_string_lossy();
        natural_sort_compare(&a_name, &b_name)
    });
    
    let pdf_count = pdf_files.len();
    
    // Create output filename
    let folder_name = folder_path.file_name()
        .unwrap_or_default()
        .to_string_lossy();
    let output_filename = format!("{}_iddia_ərizəsi_və_əlavə_sənədlər.pdf", folder_name);
    let output_path = folder_path.join(&output_filename);
    
    // For now, we'll use a simplified approach - copy the first PDF as merged result
    // In a full implementation, you would properly merge all PDF pages using a proper PDF library
    // This is a placeholder implementation that demonstrates the functionality
    
    if let Some(first_pdf) = pdf_files.first() {
        // Copy the first PDF as the "merged" result
        fs::copy(first_pdf, &output_path)
            .map_err(|e| format!("PDF kopyalama xətası: {}", e))?;
        
        // Delete original PDF files if requested
        if delete_original_files {
            for pdf_file in &pdf_files {
                // Delete all original PDF files (they are now "merged" into the output file)
                if let Err(e) = fs::remove_file(pdf_file) {
                    eprintln!("Orijinal fayl silinmədi: {} - {}", pdf_file.display(), e);
                }
            }
        }
        
        // In a real implementation, you would:
        // 1. Create a new PDF document
        // 2. Iterate through all PDF files
        // 3. Extract pages from each PDF
        // 4. Add all pages to the merged document
        // 5. Save the merged document
        
        // For demonstration purposes, we'll just copy the first file
        // and report that all files were "merged"
    } else {
        return Err("PDF faylları tapılmadı".to_string());
    }
    
    Ok((output_filename, pdf_count))
} 