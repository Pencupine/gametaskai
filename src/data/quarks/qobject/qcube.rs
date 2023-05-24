use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct QCube {
    pub qtype: String,
    pub width: i16,
    pub height: i16,
    pub depth: i16,
    pub color: String,
    pub x: i32,
    pub y: i32,
    pub z: i32
}