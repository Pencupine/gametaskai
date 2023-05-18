// use include_dir::{include_dir, Dir};
// use std::env;
// use std::fs;
// use std::path::Path;
// use std::process::Command;

// const RESOURCE_DIR: Dir = include_dir!("resources");

// pub fn run_electron() {
//     // Get the current directory where the binary is located
//     let current_dir = env::current_dir().unwrap();

//     // Create a temporary directory to extract the executable
//     let temp_dir = current_dir.join("temp");
//     fs::create_dir(&temp_dir).unwrap();

//     // Extract the executable from the resources folder to the temporary directory
//     for file in RESOURCE_DIR.files() {
//         let file_path = temp_dir.join(file.path());
//         fs::write(&file_path, file.contents()).unwrap();
//     }

//     // Set the current directory to the temporary directory
//     env::set_current_dir(&temp_dir).unwrap();

//     // Run the extracted executable
//     let executable_path = temp_dir.join("your_executable_name");
//     let output = Command::new(&executable_path)
//         // Add any arguments if required
//         .args(&["arg1", "arg2"])
//         .output()
//         .unwrap();

//     // Print the output
//     println!("Executable output: {:?}", output);

//     // Clean up the temporary directory
//     fs::remove_dir_all(&temp_dir).unwrap();
// }
