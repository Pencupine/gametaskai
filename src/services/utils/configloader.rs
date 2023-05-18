use std::fs::File;
use std::io::Read;
use std::env;

use crate::services::utils::config::Config;


pub fn load_config() -> Config {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("{}", current_dir.display());
    let config_relative_path = "/Library/Preferences/gametaskui/config.yaml";
    let config_path = current_dir.join(config_relative_path);
    println!("Config path {}", config_path.display());

    let mut file = File::open(config_path).expect("Failed to open config.yaml");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read config.yaml");
    serde_yaml::from_str(&contents).expect("Failed to parse YAML")
}