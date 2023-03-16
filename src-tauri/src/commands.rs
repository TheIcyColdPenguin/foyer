use chrono::{DateTime, Utc};
use exif::Tag;
use img_parts::{jpeg::Jpeg, png::Png, ImageEXIF};
use rusqlite::{params, Connection};

use crate::{string_error::Nope, types::Photo};
use std::{fs, path::PathBuf, sync::Mutex, time::SystemTime};

macro_rules! skip_fail {
    ($x:expr) => {
        match $x {
            Ok(res) => res,
            Err(e) => {
                println!("Skipping error {:?}", e.to_string());
                continue;
            }
        }
    };
}

fn is_image(ext: &str) -> bool {
    ["png", "jpg", "gif", "tiff", "jpeg", "bmp"].contains(&ext)
}

fn get_current_timestamp() -> String {
    DateTime::<Utc>::from(SystemTime::now())
        .format("%+")
        .to_string()
}

fn get_exif_timestamp(path: &PathBuf) -> Result<String, String> {
    let read_file = fs::read(&path).nope()?;
    let extension = path.extension().map(|s| s.to_string_lossy().to_string());

    let exif_raw = match extension.as_deref() {
        Some("jpeg") | Some("jpg") => {
            let jpeg = Jpeg::from_bytes(read_file.into()).nope()?;
            jpeg.exif()
        }
        Some("png") => {
            let png = Png::from_bytes(read_file.into()).nope()?;
            png.exif()
        }
        _ => None,
    };

    let Some(raw_data) = exif_raw else {
        return Err("No data found".into());
    };

    let exifreader = exif::Reader::new();
    let exif = exifreader.read_raw(raw_data.clone().into()).nope()?;

    let tag = exif
        .get_field(Tag::DateTime, exif::In::PRIMARY)
        .ok_or("field not found".to_string())
        .nope()?;

    Ok(tag.display_value().to_string())
}

fn get_exif_from_file_metadata(path: &PathBuf) -> Result<String, String> {
    let file_opened = fs::File::open(&path).nope()?;
    let metadata = file_opened.metadata().nope()?;
    let datetime: DateTime<Utc> = DateTime::from(metadata.created().nope()?);

    Ok(datetime.format("%+").to_string())
}

fn get_exif(path: &PathBuf) -> String {
    if let Ok(timestamp) = get_exif_timestamp(path) {
        return timestamp;
    }

    if let Ok(timestamp) = get_exif_from_file_metadata(path) {
        return dbg!(timestamp);
    }

    get_current_timestamp()
}

#[derive(Default, Debug)]
pub struct Database(pub Mutex<Option<Connection>>);

#[tauri::command]
pub async fn is_connected_db(state: tauri::State<'_, Database>) -> Result<bool, String> {
    let lock = state.0.lock().nope()?;
    Ok(lock.is_some())
}
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
    let mut file_details = Vec::with_capacity(folders.len());
    for folder in folders.iter() {
        for file in skip_fail!(fs::read_dir(folder)) {
            let file = skip_fail!(file);
            let file_path = file.path();

            if !file_path.is_file() {
                continue;
            }

            let Some(extension) = file_path.extension().map(|s| s.to_string_lossy().to_string()) else {
                continue;
            };

            if !is_image(&extension) {
                continue;
            }

            let timestamp = get_exif(&file_path);

            let data = skip_fail!(fs::read(&file_path));

            file_details.push((data, extension, timestamp));
        }
    }

    let Some(ref mut conn) = *state.0.lock().nope()? else {
        return Err("connection not established".into());
    };

    let mut statement = conn
        .prepare_cached("INSERT INTO photos(img_data, extension, timestamp) VALUES (?, ?, ?);")
        .nope()?;

    for (file, ext, timestamp) in file_details.iter() {
        skip_fail!(statement.execute(params![
            file,      // binary photo data
            ext,       // file extension
            timestamp  // photo created timestamp, or current timestamp if that doesnt exist
        ]));
    }

    Ok(())
}

#[tauri::command]
pub async fn fetch_photos_after(
    offset: u64,
    state: tauri::State<'_, Database>,
) -> Result<Vec<Photo>, String> {
    let Some(ref mut conn) = * state.0.lock().nope()? else {
        return Err("connection not established".into());
    };
    let mut statement = conn
        .prepare_cached(
            "SELECT id, extension, timestamp FROM photos ORDER BY timestamp DESC LIMIT 50 OFFSET ?;",
        )
        .nope()?;

    let photos: Vec<_> = statement
        .query_map([offset], |row| {
            Ok(Photo {
                id: row.get(0)?,
                ext: row.get(1)?,
                timestamp: row.get(2)?,
            })
        })
        .nope()?
        .filter_map(|photo| photo.ok())
        .collect();

    Ok(photos)
}
