#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod img_uri_handler;
mod string_error;
mod types;

use std::collections::HashMap;

use commands::{connect_db, fetch_photos, upload_photos, Database};
use img_uri_handler::handle_image_request;
use string_error::Nope;
use tauri::{
    http::{method::Method, ResponseBuilder, Uri},
    Manager,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fix_path_env::fix()?;

    tauri::Builder::default()
        .register_uri_scheme_protocol("foyerimg", handle_image_request)
        .invoke_handler(tauri::generate_handler![
            connect_db,
            upload_photos,
            fetch_photos
        ])
        .manage(Database::default())
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");

    Ok(())
}
