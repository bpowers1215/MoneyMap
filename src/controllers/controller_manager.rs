// src/controllers/controller_manager.rs

/// Controller Manager
/// Store Controllers in one place

//Import Controller Modules
use super::test_controller::TestController;
use super::accounts_controller::AccountsController;
use super::account_statements_controller::AccountStatementsController;
use super::money_maps_controller::MoneyMapsController;
use super::money_map_users_controller::MoneyMapUsersController;
use super::users_controller::UsersController;

/// Controller Manager
#[derive(Clone)]
pub struct ControllerManager {
    pub test_controller: TestController,
    pub accounts_controller: AccountsController,
    pub account_statements_controller: AccountStatementsController,
    pub money_maps_controller: MoneyMapsController,
    pub money_map_users_controller: MoneyMapUsersController,
    pub users_controller: UsersController
}
