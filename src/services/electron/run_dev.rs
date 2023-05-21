use std::sync::{Arc, Mutex};
use std::{process::Command};
use std::path::Path;
// use std::thread;

use crate::services::utils::config::{ElectronConfig};
use crate::services::electron::electron_app::e_app::EApp;

#[allow(unused_variables)]
pub fn run_electron_dev(electron_config: ElectronConfig, eapp: Arc<Mutex<EApp>>) -> std::io::Result<()> {
    // let mut child = 
    Command::new("electron")
        .arg(".")
        .current_dir(Path::new(&format!("./{}", electron_config.path)))
        .spawn()
        .expect("Failed to start electron application in development environment!!!");

    // let child_thread = thread::spawn(move || {
    //     let exit_status = child.wait()
    //         .expect("Failed to wait for child process");

    //     let mut eapp = eapp.lock().unwrap();
    //     eapp.exit_electron_app();
    //     println!("GTAI App: {}", eapp.init);
    // });

    // child_thread.join().expect("Failed to join the child thread");

    Ok(())
}
