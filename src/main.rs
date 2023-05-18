// use serde::{Deserialize, Serialize};
// use serde_json::{Result, Value};
// use std::fs::{File, OpenOptions};
// use std::io::{BufReader, BufWriter};
// use std::path::Path;

// mod data {

// }

mod data;
mod services;

use services::gametask::master::master;

fn main() {
    println!("Rust Application Start");

    master();
}