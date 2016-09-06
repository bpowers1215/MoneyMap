// src/common/data_access.rs

/// Data Access Object

// Import Modules
// Utilities
use ::common::config::Config;
// Controllers
use ::controllers::controller_manager::ControllerManager;

#[derive(Clone)]
pub struct ServerData {
    pub config: Config,
    pub controller_manager: ControllerManager
}
