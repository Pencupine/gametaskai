// use std::{process::Command, io::Write};
// use std::io::Result;
// use std::path::Path;
// use tempfile::NamedTempFile;

// use include_dir::{include_dir, Dir};

// use crate::services::utils::config::{ElectronConfig};

// const RESOURCE_DIR: Dir = include_dir!("resources");

// pub fn run_electron(electron_config : ElectronConfig) -> Result<()> {

    
//     // Execute the Electron app using the "electron" command
//     if electron_config.is_dev {
//         Command::new("electron")
//         .arg(".")
//         .current_dir(Path::new(&format!("./{}", electron_config.path)))
//         .spawn()?;
//     } else {
//         println!("Hii");
//         // Extract the executable file from the resource directory
//         let executable_path = extract_executable_from_resources("resources/public/gametaskui");

//         // Run the extracted executable
//         let exit_status = Command::new(executable_path)
//             .status()
//             .unwrap_or_else(|_| panic!("Failed to execute the extracted executable"));

//         // Handle the exit status as needed
//         if exit_status.success() {
//             println!("Executable ran successfully");
//         } else {
//             eprintln!("Executable failed with status: {}", exit_status);
//         }
//     }
//     Ok(())
// }


// // Function to extract the executable file from the resources directory
// fn extract_executable_from_resources(resource_path: &str) -> String {
//     let executable_bytes = RESOURCE_DIR
//         .get_file(resource_path)
//         .expect("Failed to get the executable from the resource directory")
//         .contents();

   
//     let mut executable_file = NamedTempFile::new()
//         .expect("Failed to create a temporary file for the executable");

//     executable_file
//         .write(executable_bytes)
//         .expect("Failed to write executable bytes to the temporary file");

//     // Return the path of the extracted executable
//     executable_file.into_temp_path().to_string_lossy().to_string()
// }