use fs_extra::dir::{copy, CopyOptions};
use std::env;
use std::fs;
use std::path::{Path};

use include_dir::include_dir;

fn main() {
    let target_env = env::var("TARGET_ENV").unwrap_or_else(|_| String::from("mac"));

    println!("cargo:warning=BUILDING CARGO EXECUTABLE");

    let electron_app_path = "electron";

    if Path::new("./resources/public").exists() {
        fs::remove_dir_all("./resources/public")
            .expect("Failed to delete the resource folder");
        println!("cargo:warning=Resources/public folder deleted");
    } else {
        println!("cargo:warning=Resources/public folder does not exist");
    }

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

    println!("Installing new Electron build.");
    let copy_options = CopyOptions {
        content_only: true,
        ..Default::default()
    };
    let current_dir = env::current_dir().expect("Failed to get current directory");
    
    let electron_dir = Path::new("electron/dist/mac-arm64/gametaskui.app/Contents/MacOS/");
    let electron_absolute_dir = current_dir.join(electron_dir);

    if electron_absolute_dir.exists() {
        println!("cargo:warning=Found electron distributable {}", electron_absolute_dir.display());
        
        let dest_relative_path = Path::new("resources/public");
        let dest_absolute_path = current_dir.join(dest_relative_path);

        if let Err(err) = copy(electron_absolute_dir.as_path(), dest_absolute_path, &copy_options) {
            eprintln!("cargo:warning=Failed to copy Electron resources: {}", err);
        }

    } else {
        println!("cargo:warning=Electron dir {}", electron_dir.display());
        println!("cargo:warning=Coudn't find electron distributable {}", electron_absolute_dir.display());
    }
    
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
    let resource_dir = include_dir!("resources");

    // Generate the Rust code to embed the resource directory
    resource_dir
        .as_variable("EMBEDDED_RESOURCES")
        .to_file("src/embedded_resources.rs")
        .expect("Failed to write embedded resource file");

}
