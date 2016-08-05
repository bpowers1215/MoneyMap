// src/controllers/controller_manager.rs

/// Controller Manager
/// Store Controllers in one place

//Import Controller Modules
use super::test_controller::TestController;
use super::users_controller::UsersController;

/// Controller Manager
#[derive(Clone)]
pub struct ControllerManager {
    pub test_controller: TestController,
    pub users_controller: UsersController
}