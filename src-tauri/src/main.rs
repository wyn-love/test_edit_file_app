#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use edit_file_name::OpenDir;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn edit_file(dir: &str, prefix: String) {
    
    let mut opt = OpenDir::new(dir.into(), prefix);

    opt.edit_files_name().expect("edit file name failed");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![edit_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
