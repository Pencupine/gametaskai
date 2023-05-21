use std::fs;
use std::path::Path;
use warp::reply::Reply;
use serde_json;

use crate::data::quarks::qdummy::dum::Dum;

const DATA_DIR: &str = "/Library/Application Support/gametaskai";

#[derive(Debug)]
struct IOError;

impl warp::reject::Reject for IOError {}

fn handle_io_error(err: std::io::Error) -> warp::Rejection {
    eprintln!("I/O error: {}", err);
    warp::reject::custom(IOError)
}

pub async fn read_from_file() -> Result<impl Reply, warp::Rejection> {
    if !Path::new(DATA_DIR).exists() {
        fs::create_dir_all(DATA_DIR).map_err(handle_io_error)?;
    }
    let file_path = format!("{}/gtai_data.json", DATA_DIR);
    let file_content = fs::read_to_string(&file_path).map_err(handle_io_error)?;
    let data: Dum = serde_json::from_str(&file_content).map_err(|err| {
        eprintln!("error deserializing data: {}", err);
        warp::reject::not_found()
    })?;
    Ok(warp::reply::json(&data))
}

pub async fn write_to_file(data: Dum) -> Result<impl Reply, warp::Rejection> {
    if !Path::new(DATA_DIR).exists() {
        fs::create_dir_all(DATA_DIR).map_err(handle_io_error)?;
    }
    let file_path = format!("{}/gtai_data.json", DATA_DIR);
    let data_to_write = serde_json::to_string(&data).map_err(|err| {
        eprintln!("error serializing data: {}", err);
        warp::reject::not_found()
    })?;
    fs::write(&file_path, data_to_write).map_err(handle_io_error)?;
    Ok(format!("Data written successfully"))
}
