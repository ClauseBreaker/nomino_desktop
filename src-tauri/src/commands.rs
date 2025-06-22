/*!
 * Tauri Commands Module
 * 
 * This module contains all the backend commands that can be invoked from the frontend.
 * It handles file system operations, Excel processing, and folder renaming operations
 * with support for Azerbaijani alphabet sorting.
 */

use calamine::{open_workbook, DataType, Reader, Xlsx};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::Duration;
use tauri::{command, Window};
use tokio::time::sleep;

#[cfg(windows)]
use windows::core::PCWSTR;
#[cfg(windows)]
use windows::Win32::UI::Shell::StrCmpLogicalW;

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
// Basic Commands
// ================================================================================================

/// Simple greeting command for testing backend connectivity
#[command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// ================================================================================================
// File System Operations
// ================================================================================================

/// Retrieves all files in a specified directory
#[command]
pub async fn get_files_in_directory(path: String) -> Result<Vec<FileInfo>, String> {
    let dir_path = Path::new(&path);
    
    if !dir_path.exists() {
        return Err("Directory does not exist".to_string());
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
        return Err("Directory does not exist".to_string());
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
        return Err("Directory does not exist".to_string());
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
        return Err("Directory does not exist".to_string());
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
                                    return Err(format!("Failed to rename {}: {}", old_name, e));
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
        return Err("Directory does not exist".to_string());
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
                                    return Err(format!("Failed to rename folder {}: {}", old_name, e));
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

/// Main folder renaming operation using Excel data
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
) -> Result<Vec<String>, String> {
    let source_dir = Path::new(&source_path);
    let dest_dir = Path::new(&destination_path);
    
    // Validate directories
    if !source_dir.exists() {
        return Err("Source directory does not exist".to_string());
    }
    
    if !dest_dir.exists() {
        return Err("Destination directory does not exist".to_string());
    }
    
    // Send initial progress
    emit_progress(&window, 0, folders.len(), "Reading Excel file...", "Loading names from Excel");
    
    // Read names from Excel file
    let excel_names = read_excel_names(&excel_path, start_row, &column)?;
    
    if excel_names.is_empty() {
        return Err("No names found in Excel file".to_string());
    }
    
    emit_progress(&window, 0, folders.len(), "Starting process...", &format!("{} folders to process", folders.len()));
    
    let mut results = Vec::new();
    
    // Process each folder with corresponding Excel name
    for (index, folder_name) in folders.iter().enumerate() {
        let current = index + 1;
        
        emit_progress(&window, current, folders.len(), &format!("Processing folder: {}", folder_name), &format!("{}/{} folders", current, folders.len()));
        
        let old_folder_path = source_dir.join(folder_name);
        
        if !old_folder_path.exists() {
            let error_msg = format!("❌ Error: Folder '{}' not found", folder_name);
            results.push(error_msg.clone());
            
            emit_process_result(&window, false, &error_msg, folder_name, "");
            continue;
        }
        
        // Get new name from Excel
        let new_name = if index < excel_names.len() {
            &excel_names[index]
        } else {
            let error_msg = format!("❌ Error: No Excel name for folder '{}' (row {})", folder_name, start_row + index as u32);
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
                let success_msg = format!("✅ Success: '{}' → '{}'", folder_name, safe_new_name);
                results.push(success_msg.clone());
                
                emit_process_result(&window, true, &success_msg, folder_name, &safe_new_name);
            }
            Err(e) => {
                let error_msg = format!("❌ Error: Failed to move '{}': {}", folder_name, e);
                results.push(error_msg.clone());
                
                emit_process_result(&window, false, &error_msg, folder_name, &safe_new_name);
            }
        }
    }
    
    // Send completion
    emit_progress(&window, folders.len(), folders.len(), "Completed!", "All folders processed");
    
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
        "PDF creation scheduled: {} files -> {} (title: {})",
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
        result = "Unnamed_Folder".to_string();
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
        .map_err(|e| format!("Failed to open Excel file: {}", e))?;
    
    let worksheet_name = workbook.worksheet_names().first()
        .ok_or("No worksheets found in Excel file")?
        .clone();
    
    let range = workbook.worksheet_range(&worksheet_name)
        .ok_or("Failed to get worksheet range")?
        .map_err(|e| format!("Failed to read worksheet: {}", e))?;
    
    let column_index = column_letter_to_index(column)?;
    let mut names = Vec::new();
    
    // Read from start_row (1-indexed) to end of data
    for row in (start_row - 1)..range.height() as u32 {
        if let Some(cell) = range.get_value((row as usize, column_index)) {
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
            return Err(format!("Invalid column letter: {}", column));
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
                .map_err(|e| format!("Failed to remove source folder: {}", e))?;
            Ok(())
        }
    }
}

/// Recursively copies a directory
fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<(), String> {
    fs::create_dir_all(destination)
        .map_err(|e| format!("Failed to create destination directory: {}", e))?;
    
    for entry in fs::read_dir(source)
        .map_err(|e| format!("Failed to read source directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let source_path = entry.path();
        let dest_path = destination.join(entry.file_name());
        
        if source_path.is_dir() {
            copy_dir_recursive(&source_path, &dest_path)?;
        } else {
            fs::copy(&source_path, &dest_path)
                .map_err(|e| format!("Failed to copy file: {}", e))?;
        }
    }
    
    Ok(())
} 