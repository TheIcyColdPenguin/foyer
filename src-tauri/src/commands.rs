use rusqlite::{self, Connection};

use std::{fs, sync::Mutex};

#[derive(Default, Debug)]
pub struct Database(pub Mutex<Option<Connection>>);

#[tauri::command]
pub async fn connect_db(db_path: String, state: tauri::State<'_, Database>) -> Result<(), String> {
    let connection = rusqlite::Connection::open(db_path).map_err(|e| e.to_string())?;

    connection
        .execute_batch(include_str!("initialise.sql"))
        .map_err(|e| e.to_string())?;

    *state.0.lock().unwrap() = Some(connection);

    Ok(())
}

#[tauri::command]
pub async fn upload_photos(
    folders: Vec<String>,
    state: tauri::State<'_, Database>,
) -> Result<(), String> {
    let files = folders.iter().map(|folder| fs::read_dir(folder));

    let Some(ref mut conn) = * state.0.lock().map_err(|e| e.to_string())? else {
        return Err("connection not established".into());
    };

    Ok(())
}
