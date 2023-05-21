use std::sync::{Arc, Mutex};

use crate::services::electron::run_dev::run_electron_dev;
use crate::services::electron::run_prod::run_electron_prod;
use crate::services::utils::config::ElectronConfig;
use crate::services::electron::electron_app::e_app::EApp;

#[allow(unused_variables)]
pub fn run_electron(electron_config: ElectronConfig) {

    let eapp = Arc::new(Mutex::new(EApp::new()));

    if electron_config.is_dev {
        let run_status = run_electron_dev(electron_config, eapp.clone());
        match run_status {
            Ok(child) => { println!("Electron development version running..."); }
            Err(e) => { println!("Electron development version failed to start!!! :("); }
        }
    } else {
        run_electron_prod();
    }
}
