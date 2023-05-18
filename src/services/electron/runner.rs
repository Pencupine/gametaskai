use std::env;
// use std::fs::File;
// use std::io::prelude::*;
use std::path::{ PathBuf};
use std::process::{Command};
use std::env::current_exe;
use tempfile::NamedTempFile;

use include_dir::{include_dir, Dir};

const RESOURCE_DIR: Dir = include_dir!("resources");

pub fn run_electron() {
    // Get the path to the directory containing the Rust executable
    let current_dir = current_exe()
        .map(|path| path.parent().unwrap().to_owned())
        .expect("Failed to get current directory");

    println!("Current Dir {}", current_dir.display());

    // Prompt the user to select the Electron app executable
    select_electron_app();
    // let electron_app_path = select_electron_app().expect("Failed to select Electron app");
    // println!("Electron_app_path {}", electron_app_path.display());

    // // Read the Electron app executable into a byte array
    // let mut electron_app = Vec::new();
    // let mut file = File::open(&electron_app_path).expect("Failed to open Electron app");
    // file.read_to_end(&mut electron_app).expect("Failed to read Electron app");

    // // Create the Rust executable
    // let mut output_file = File::create(current_dir.join("rust_app")).expect("Failed to create Rust executable");

    // // Embed the Electron app as a byte array in the Rust executable
    // output_file
    //     .write_all(&electron_app)
    //     .expect("Failed to embed Electron app");

    // // Run the embedded Electron app
    // let electron_app_executable = current_dir.join("rust_app");

    // let output = Command::new(&electron_app_executable)
    //     .current_dir(current_dir)
    //     .output()
    //     .expect("Failed to run Electron app");

    // Handle the output as needed
    // ...

    // Additional Rust program logic...
}

fn select_electron_app(){
    println!("Finding Electron app");

    get_folder_content();
    // Use a file dialog to prompt the user to select the Electron app executable
    let executable_bytes = RESOURCE_DIR
        .get_file("resources/gametaskui")
        .expect("Failed to get the executable from the resource directory")
        .contents();

    println!("Executable bytes {}",executable_bytes.len());

  
    let mut executable_file = NamedTempFile::new()
        .expect("Failed to create a temporary file for the executable");

    // executable_file
    //     .write(executable_bytes)
    //     .expect("Failed to write executable bytes to the temporary file");

    // Return the path of the extracted executable
    // executable_file.into_temp_path().to_string_lossy().to_string()
    // // If the user selected a file, return its path
    // if let Some(file_path) = file_dialog {
    //     Some(PathBuf::from(file_path))
    // } else {
    //     None
    // }
}

fn get_folder_content () {
    // if let Ok(current_exe) = env::current_exe() {
    //     let resource_dir = 

    //     match resource_dir {
    //         Some(dir) => {
    //             if dir.exists() {
    //                 println!("Resource folder path: {:?}", dir);
    //             } else {
    //                 println!("Resource folder does not exist.");
    //             }
    //         }
    //         None => {
    //             println!("Unable to determine resource folder path.");
    //         }
    //     }
    // } else {
    //     println!("Unable to retrieve the current binary path.");
    // }

}