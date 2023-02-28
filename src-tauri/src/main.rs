#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;

use commands::{connect_db, Database};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fix_path_env::fix()?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect_db])
        .manage(Database::default())
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");

    Ok(())
}
