use std::collections::HashMap;

use tauri::{
    http::{method::Method, Request, Response, ResponseBuilder, Uri},
    AppHandle, Runtime,
};

use crate::{commands::Database, string_error::Nope};

use tauri::Manager;

pub fn handle_image_request<R: Runtime>(
    app: &AppHandle<R>,
    req: &Request,
) -> Result<Response, Box<dyn std::error::Error>> {
    let no_img = ResponseBuilder::new().status(404).body(vec![]);

    if req.method() != Method::GET {
        return no_img;
    }
    let uri = Uri::try_from(req.uri())?;

    let Some(query_params) = uri.query() else {
        return no_img;
    };
    let pairs: HashMap<&str, &str> = query_params
        .split('&')
        .filter_map(|each| each.split_once('='))
        .collect();

    let Some(id) = pairs.get("id") else {
        return no_img;
    };

    let state: tauri::State<'_, Database> = app.state();
    let Some(ref mut conn) = * state.0.lock().nope()? else {
        return Err("connection not established".into());
    };

    let mut statement = conn.prepare_cached("SELECT img_data FROM photos WHERE id=?;")?;

    let photos = statement.query_row([id], |row| row.get::<_, Vec<u8>>(0))?;

    ResponseBuilder::new().mimetype("image/png").body(photos)
}
