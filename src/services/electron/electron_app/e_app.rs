#[derive(Debug, PartialEq, Eq)]
pub enum RunStatus {
    // OFF,
    RUNNING
}

#[derive(Debug, PartialEq, Eq)]
pub struct EApp {
    pub init: bool,
    pub status: RunStatus,
}


impl EApp {
    pub fn new() -> Self {
        EApp {
            init: true,
            status: RunStatus::RUNNING,
        }
    }

    // pub fn exit_electron_app (&mut self) {
    //     self.init = false;
    //     self.status = RunStatus::OFF;
    // }
}
