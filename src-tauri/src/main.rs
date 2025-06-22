/*!
 * Nomino - Folder Renaming Application
 * 
 * A cross-platform desktop application for bulk folder renaming operations
 * using Excel data with support for Azerbaijani alphabet sorting.
 * 
 * Originally created by ClauseBreaker (https://github.com/ClauseBreaker) in 2025
 * 
 * Built with:
 * - Tauri (Rust backend)
 * - SvelteKit (Frontend)
 * - Calamine (Excel processing)
 * 
 * License: MIT
 */

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Import application modules
mod commands;

// Import command functions
use commands::{
    ProcessState,
    greet,
    debug_folder_structure,
    get_files_in_directory,
    get_folders_in_directory, 
    get_folders_with_sorting,
    get_files_with_sorting,
    rename_files,
    rename_folders,
    rename_folders_from_excel,
    rename_files_from_excel,
    rename_files_from_excel_advanced,
    read_excel_column,
    create_pdf,
    create_pdf_from_images,
    get_pdf_subfolders,
    copy_file_to_all_subfolders,
    change_pdf_dates,
    merge_pdf_files,
    pause_process,
    resume_process,
    stop_process,
    get_process_status
};

/**
 * Application entry point
 * 
 * Initializes the Tauri application with all available commands
 * and starts the main event loop.
 */
fn main() {
    // Configure and build the Tauri application
    let app = tauri::Builder::default()
        .manage(ProcessState::new())
        .invoke_handler(tauri::generate_handler![
            // Basic utilities
            greet,
            debug_folder_structure,
            
            // File system operations
            get_files_in_directory,
            get_folders_in_directory,
            get_folders_with_sorting,
            get_files_with_sorting,
            
            // Excel integration
            read_excel_column,
            
            // Renaming operations
            rename_files,
            rename_folders,
            rename_folders_from_excel,
            rename_files_from_excel,
            rename_files_from_excel_advanced,
            
            // Document operations
            create_pdf,
            
            // PDF creation operations
            create_pdf_from_images,
            get_pdf_subfolders,
            
            // File copy operations
            copy_file_to_all_subfolders,
            
            // PDF date change operations
            change_pdf_dates,
            
            // PDF merger operations
            merge_pdf_files,
            
            // Process control operations
            pause_process,
            resume_process,
            stop_process,
            get_process_status
        ])
        .build(tauri::generate_context!());

    // Handle application startup errors
    match app {
        Ok(app) => {
            // Run the application
            app.run(|_app_handle, event| match event {
                tauri::RunEvent::ExitRequested { api, .. } => {
                    // Prevent the default close behavior and hide the app
                    api.prevent_exit();
                }
                _ => {}
            });
        }
        Err(error) => {
            eprintln!("Failed to start application: {}", error);
            std::process::exit(1);
        }
    }
} 