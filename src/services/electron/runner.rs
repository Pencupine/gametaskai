use crate::services::electron::run_dev::run_electron_dev;
use crate::services::electron::run_prod::run_electron_prod;
use crate::services::utils::config::ElectronConfig;

pub fn run_electron_dev(electron_config: ElectronConfig) {

    if electron_config.is_dev {
        run_electron_dev(electron_config)
    } else {
        run_electron_prod();
    }
}
