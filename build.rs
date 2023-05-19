use fs_extra::dir::{copy, CopyOptions};
use std::env;
use std::fs;
use std::path::{Path};

fn main() {
    let target_env = env::var("TARGET_ENV").unwrap_or_else(|_| String::from("mac"));

    println!("cargo:warning=BUILDING CARGO EXECUTABLE");

    let electron_app_path = "electron";

    // STEP 1
    if Path::new("./resources").exists() {
        fs::remove_dir_all("./resources")
            .expect("Failed to delete the resource folder");
        println!("cargo:warning=Resources folder deleted");
    } else {
        println!("cargo:warning=Resources folder does not exist");
    }
        // STEP 2

    println!("cargo:warning=Building for target env: {}", target_env);

    let output = std::process::Command::new("gulp")
        .arg(&format!("release-{}", target_env))
        .current_dir(Path::new(&format!("./{}", electron_app_path)))
        .output()
        .expect("Failed to execute gulp");

    if !output.status.success() {
        panic!(
            "Failed to build Electron app: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    } else {
        println!("cargo:warning=Successfully built Electron.")
    }
    // STEP 3
    println!("Installing new Electron build.");
    let copy_options = CopyOptions {
        content_only: true,
        ..Default::default()
    };
    let current_dir = env::current_dir().expect("Failed to get current directory");
    
    let electron_dir = Path::new("electron/dist/mac-arm64");
    let electron_absolute_dir = current_dir.join(electron_dir);

    if electron_absolute_dir.exists() && electron_absolute_dir.is_dir() {
        println!("cargo:warning=Found electron distributable {}", electron_absolute_dir.display());
        
        let dest_relative_path = Path::new("resources");
        let dest_absolute_path = current_dir.join(dest_relative_path);

        if let Err(err) = copy(electron_absolute_dir.as_path(), dest_absolute_path, &copy_options) {
            eprintln!("cargo:warning=Failed to copy Electron resources: {}", err);
        }

    } else {
        println!("cargo:warning=Electron dir {}", electron_dir.display());
        println!("cargo:warning=Coudn't find electron distributable {}", electron_absolute_dir.display());
    }
    
    // STEP 4
    let resources_assets_dir = Path::new("resources");
    let resources_absolute_dir = current_dir.join(electron_dir);

    if resources_assets_dir.exists() {
        println!("cargo:warning=Found resources and assets {}", resources_assets_dir.display());
        
        let out_dir = env::var("OUT_DIR").unwrap();
        let resource_dest_path = Path::new("resources");
        let dest_path = Path::new(&out_dir).join(resource_dest_path);

        if let Err(err) = copy(resources_absolute_dir.as_path(), dest_path, &copy_options) {
            eprintln!("cargo:warning=Failed to copy resources for release: {}", err);
        }

    } else {
        println!("cargo:warning=Coudn't find electron distributable {}", resources_assets_dir.display());
    } // Embed the "resources" directory as bytes
    
    // // Create a directory to 
    // let out_dir = env::var("OUT_DIR").expect("Failed to retrieve OUT_DIR");
    // let dest_dir = Path::new(&out_dir).join("resources");
    // fs::create_dir_all(&dest_dir).expect("Failed to create resource directory");

    // // Copy the executable files to the destination directory
    // copy_executable("resources/gametaskui", &dest_dir, copy_options);


}

// // Helper function to copy the executable file to the destination directory
// fn copy_executable(src_path: &str, dest_dir: &Path, copyOpt: CopyOptions) {
//     println!("cargo:warning=Copying executable into out dir");
//     let src_file = Path::new(src_path);
//     let dest_file = dest_dir.join(src_file.file_name().expect("Invalid source path"));
    
//     if let Err(err) = copy(src_file, dest_file, &copyOpt) {
//         eprintln!("cargo:warning=Failed to copy resources for release: {}", err);
//     }

//     // Make the destination file executable
//     let metadata = fs::metadata(&dest_file).expect("Failed to retrieve file metadata");
//     let mut permissions = metadata.permissions();
//     permissions.to_owned();
//     fs::set_permissions(&dest_file, permissions).expect("Failed to set file permissions");   
// }