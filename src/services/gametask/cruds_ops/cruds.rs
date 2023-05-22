use std::fs;
use std::path::{Path, PathBuf};
use warp::fs::file;
use warp::reply::Reply;
use serde_json;

use crate::data::quarks::qdummy::dum::Dum;

const DATA_DIR: &str = "gametaskai";

#[derive(Debug)]
struct IOError;

impl warp::reject::Reject for IOError {}

fn handle_io_error(err: std::io::Error) -> warp::Rejection {
    eprintln!("I/O error: {}", err);
    warp::reject::custom(IOError)
}

fn get_data_directory() -> Result<PathBuf, std::io::Error> {
    let app_support_dir = match dirs::data_dir() {
        Some(mut dir) => {
            dir.push("gametaskai");
            dir
        },
        None => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get application support directory."));
        }
    };

    Ok(app_support_dir)
}
pub async fn read_from_file() -> Result<impl Reply, warp::Rejection> {
    println!("READ FILE OPERATION");
    let data_dir = get_data_directory().map_err(handle_io_error)?;
    
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(handle_io_error)?;
    }
    
    let file_path = data_dir.join("gtai_data.json");
    println!("{}", file_path.display());
    let file_content = fs::read_to_string(&file_path).map_err(handle_io_error)?;
    
    let data: Dum = serde_json::from_str(&file_content).map_err(|err| {
        eprintln!("error deserializing data: {}", err);
        warp::reject::not_found()
    })?;
    
    Ok(warp::reply::json(&data))
}

pub async fn write_to_file(data: Dum) -> Result<impl Reply, warp::Rejection> {
    println!("Hii");
    let data_dir = get_data_directory().map_err(handle_io_error)?;
    
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(handle_io_error)?;
    }
    
    let file_path = data_dir.join("gtai_data.json");
    
    let data_to_write = serde_json::to_string(&data).map_err(|err| {
        eprintln!("error serializing data: {}", err);
        warp::reject::not_found()
    })?;

    println!("Data to be written : {}", data_to_write);
    
    println!("Write Location: {}", file_path.display());
    
    fs::write(&file_path, data_to_write).map_err(handle_io_error)?;
    
    Ok(format!("Data written successfully"))
}
