// src/main.rs

extern crate money_map;

#[macro_use]
extern crate log;
extern crate log4rs;
extern crate schedule;

// Import
// External
use schedule::Agenda;
use std::time::Duration;
// Common Utilities
use money_map::common::database::DB;
use money_map::common::config::Config;
// DAO
use money_map::dao::dao_manager::{DAOManager};
// Controllers
use money_map::controllers::controller_manager::ControllerManager;
use money_map::controllers::test_controller::TestController;
use money_map::controllers::accounts_controller::AccountsController;
use money_map::controllers::account_statements_controller::AccountStatementsController;
use money_map::controllers::money_maps_controller::MoneyMapsController;
use money_map::controllers::money_map_users_controller::MoneyMapUsersController;
use money_map::controllers::transactions_controller::TransactionsController;
use money_map::controllers::users_controller::UsersController;

fn main() {
	// Setup logging
	log4rs::init_file("config/log.toml", Default::default()).unwrap();
	info!("Executing Statement Generation Task...");

	// Load Configuration
	let configuration = Config::new();

	// Initialize Database Connection
	let db = match DB::new(configuration.clone()){
		Ok(db) => db,
		Err(e) => {
			//Cannot create database connection
			panic!("{}", e);
		}
	};

	// Initialize DAO Manager
	let dao_manager = DAOManager::new(db);

	// Initialize Controllers
	let controller_manager = ControllerManager{
		test_controller: TestController::new(dao_manager.clone()),
		accounts_controller: AccountsController::new(dao_manager.clone(), configuration.clone()),
		account_statements_controller: AccountStatementsController::new(dao_manager.clone(), configuration.clone()),
		money_maps_controller: MoneyMapsController::new(dao_manager.clone(), configuration.clone()),
		money_map_users_controller: MoneyMapUsersController::new(dao_manager.clone(), configuration.clone()),
		transactions_controller: TransactionsController::new(dao_manager.clone(), configuration.clone()),
		users_controller: UsersController::new(dao_manager.clone(), configuration.clone())
	};

    // Create new, empty agenda
    let mut scheduled_jobs = schedule::Agenda::new();

    scheduled_jobs.add(|| {
		// Generate Account Statements
		controller_manager.account_statements_controller.generate_statements();
    }).schedule("*/2 * * * * *").unwrap();

    loop {
        // Execute pending jobs
        scheduled_jobs.run_pending();

        // Sleep for 500ms
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
