// src/controllers/controller_manager.rs

/// Controller Manager
/// Store Controllers in one place

//Import Controller Modules
use super::test_controller::TestController;
use super::money_maps_controller::MoneyMapsController;
use super::users_controller::UsersController;

/// Controller Manager
#[derive(Clone)]
pub struct ControllerManager {
    pub test_controller: TestController,
    pub money_maps_controller: MoneyMapsController,
    pub users_controller: UsersController
}
