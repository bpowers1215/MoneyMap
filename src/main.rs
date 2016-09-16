// src/main.rs

#[macro_use]
extern crate nickel;

extern crate rustc_serialize;
extern crate jwt;
extern crate hyper;
extern crate crypto;
extern crate chrono;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

// Money Map Crate
extern crate money_map;

//Logging
#[macro_use]
extern crate log;
extern crate log4rs;

// Import
// External
use nickel::{MiddlewareResult, Nickel, Response, Request};
use nickel::status::StatusCode::{Forbidden};
// Common Utilities
use money_map::common::session as Session;
use money_map::common::data_access::ServerData;
use money_map::common::database::DB;
use money_map::common::config::Config;
use money_map::common::routes as Routes;
//DAO
use money_map::dao::dao_manager::{DAOManager};
//Controllers
use money_map::controllers::controller_manager::ControllerManager;
use money_map::controllers::test_controller::TestController;
use money_map::controllers::users_controller::UsersController;

fn main() {
    //Setup logging
    log4rs::init_file("config/log.toml", Default::default()).unwrap();
    info!("Initializing API");

    //Load Configuration
    let configuration = Config::new();

    //Initialize Database Connection
    let db = match DB::new(configuration.clone()){
        Ok(db) => db,
        Err(e) => {
            //Cannot create database connection
            panic!("{}", e);
        }
    };

    //Initialize DAO Manager
    let dao_manager = DAOManager::new(db);

    //Initialize Controllers
    let controller_manager = ControllerManager{
        test_controller: TestController::new(dao_manager.clone()),
        users_controller: UsersController::new(dao_manager.clone(), configuration.clone())
    };

    //Initialize Data Access object
    let server_data = ServerData{
        config: configuration,
        controller_manager: controller_manager
    };

    let mut server = Nickel::with_data(server_data);

    server.utilize(Session::authenticator);
    server.utilize(Routes::get_routes());

    server.listen("0.0.0.0:6767");
}
