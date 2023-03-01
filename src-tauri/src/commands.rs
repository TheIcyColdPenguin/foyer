use rusqlite::Connection;

use crate::{string_error::Nope, types::Photo};
use std::{fs, sync::Mutex};

#[derive(Default, Debug)]
pub struct Database(pub Mutex<Option<Connection>>);

#[tauri::command]
pub async fn connect_db(db_path: String, state: tauri::State<'_, Database>) -> Result<(), String> {
    let connection = Connection::open(db_path).nope()?;

    connection
        .execute_batch(include_str!("initialise.sql"))
        .nope()?;

    *state.0.lock().unwrap() = Some(connection);

    Ok(())
}

#[tauri::command]
pub async fn upload_photos(
    folders: Vec<String>,
    state: tauri::State<'_, Database>,
) -> Result<(), String> {
    let files: Vec<_> = folders
        .iter()
        .filter_map(|folder| fs::read_dir(folder).ok())
        .flat_map(|dir| {
            dir.filter_map(|file| file.ok())
                .filter(|file| file.path().is_file())
                .filter_map(|file| fs::read(file.path()).ok())
        })
        .collect();

    let Some(ref mut conn) = * state.0.lock().nope()? else {
        return Err("connection not established".into());
    };

    let mut statement = conn
        .prepare("INSERT INTO photos(img_data) VALUES (?);")
        .nope()?;

    for file in files.iter() {
        statement.execute([file]).nope()?;
    }

    Ok(())
}

#[tauri::command]
pub async fn fetch_photos(state: tauri::State<'_, Database>) -> Result<Vec<Photo>, String> {
    let Some(ref mut conn) = * state.0.lock().nope()? else {
        return Err("connection not established".into());
    };
    let mut statement = conn
        .prepare("SELECT id,timestamp FROM photos ORDER BY timestamp DESC LIMIT 50;")
        .nope()?;

    let photos: Vec<_> = statement
        .query_map([], |row| {
            Ok(Photo {
                id: row.get(0)?,
                timestamp: row.get(1)?,
            })
        })
        .nope()?
        .filter_map(|photo| photo.ok())
        .collect();

    Ok(photos)
}
