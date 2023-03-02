use chrono::{DateTime, Utc};
use exif::Tag;
use img_parts::{jpeg::Jpeg, png::Png, ImageEXIF};
use rusqlite::{params, Connection};

use crate::{string_error::Nope, types::Photo};
use std::{fs, path::PathBuf, sync::Mutex};

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

fn get_exif(path: &PathBuf) -> Result<String, String> {
    let read_file = fs::read(&path).nope()?;
    let extension = path.extension().map(|s| s.to_string_lossy().to_string());

    let mut exif_raw = match extension.as_deref() {
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

    let datetime = loop {
        match exif_raw {
            Some(ref raw_data) => {
                let exifreader = exif::Reader::new();
                let maybe_exif = exifreader.read_raw(raw_data.clone().into());
                let Ok(exif) = maybe_exif else {
                    exif_raw = None;
                    continue;
                };

                let Some(tag) = exif.get_field(Tag::DateTime, exif::In::PRIMARY) else {
                    exif_raw=None;
                    continue;
                };

                break Ok(tag.display_value().to_string());
            }
            None => {
                let file_opened = fs::File::open(&path).nope()?;
                let metadata = file_opened.metadata().nope()?;
                let datetime: DateTime<Utc> = DateTime::from(metadata.created().nope()?);

                break Ok(datetime.format("%+").to_string());
            }
        }
    };

    datetime
}

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

            let data = skip_fail!(fs::read(&file_path));

            let timestamp = get_exif(&file_path).unwrap_or("".into());

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
            file, // binary photo data
            ext,  // file extension
            if timestamp.is_empty() {
                "datetime('now')"
            } else {
                timestamp
            }
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
