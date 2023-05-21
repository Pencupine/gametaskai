use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Dum {
    pub name: String
}