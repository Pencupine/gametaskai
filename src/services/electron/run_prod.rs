// use std::env;
// use std::fs::{self, File};
// use std::io::Write;
// use std::os::unix::fs::PermissionsExt;
// use std::path::{Path, PathBuf};
// use std::process::{Command, exit};

pub fn run_electron_prod() {
//     let current_dir = env::current_dir().expect("Failed to retrieve current directory");

//     let relative_path = "gametaskui.app";
//     let absolute_path = current_dir.join(relative_path);

//     // Create a temporary directory for the app
//     let temp_dir = env::temp_dir().join("electron_app");
//     fs::create_dir_all(&temp_dir).expect("Failed to create temporary directory");

//     // Copy the app bundle to the temporary directory
//     let app_bundle_path = temp_dir.join("gametaskui.app");
//     copy_directory(&absolute_path, &app_bundle_path).expect("Failed to copy app bundle");

//     // Set the execute permissions on the app executable
//     #[cfg(unix)]
//     {
//         set_execute_permissions(app_bundle_path.join("Contents/MacOS/gametaskui"));
//     }
//     #[cfg(not(unix))]
//     {
//         // Implement similar logic for other platforms if needed
//     }

//     // // Run the app file
//     // let app_executable_path = app_bundle_path.join("Contents/MacOS/gametaskui");

//     println!("App executable path: {:?}", app_bundle_path);

//     // Check if gametaskui.app exists
//     if app_bundle_path.exists() {
//         println!("gametaskui.app exists");
//     } else {
//         println!("gametaskui.app does not exist");
//     }
//     let status = Command::new(&app_bundle_path)
//         .status()
//         .expect("Failed to execute Electron app");

//     // Check the execution status
//     if !status.success() {
//         eprintln!("Execution of Electron app failed");
//     }

//     // Clean up the temporary directory
//     // fs::remove_dir_all(&temp_dir).expect("Failed to remove temporary directory");
//     // Uncomment the above line if you want to remove the temporary directory after executing the app.
//     // Make sure to handle errors appropriately if the removal fails.
// }

// fn copy_directory(src: &Path, dest: &Path) -> std::io::Result<()> {
//     if src.is_dir() {
//         fs::create_dir(dest)?;

//         for entry in fs::read_dir(src)? {
//             let entry = entry?;
//             let file_name = entry.file_name();
//             let dest_path = dest.join(file_name);

//             if entry.file_type()?.is_dir() {
//                 copy_directory(&entry.path(), &dest_path)?;
//             } else {
//                 fs::copy(&entry.path(), &dest_path)?;
//             }
//         }
//     }

//     Ok(())
// }

// fn set_execute_permissions(file_path: PathBuf) {
//     if let Ok(metadata) = fs::metadata(&file_path) {
//         let mut permissions = metadata.permissions();
//         let mut mode = permissions.mode();
//         mode |= 0o111; // Add execute permission
//         permissions.set_mode(mode);
//         fs::set_permissions(&file_path, permissions).expect("Failed to set execute permissions");
//     }
}
