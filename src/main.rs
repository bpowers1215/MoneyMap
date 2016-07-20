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

// Todo Crate
extern crate money_map;

//Logging
#[macro_use]
extern crate log;
extern crate log4rs;

// Nickel
use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};
use nickel::status::StatusCode::{self, Forbidden};

// bson
use bson::{Bson, Document};
use bson::oid::ObjectId;

// rustc_serialize
use rustc_serialize::json::{Json, ToJson};
use rustc_serialize::base64;
use rustc_serialize::base64::{FromBase64};

//Common Utilities
use money_map::common::database::DB as DB;
use money_map::common::config::Config as Config;
use money_map::common::api_result::{ApiResult, JsonEncoder};

//DAO
use money_map::dao::dao_manager::{DAOManager};
use money_map::dao::user_dao::UserDAO;
use money_map::dao::money_map_dao::MoneyMapDAO;

//Controllers
use money_map::controllers::test_controller::TestController;
use money_map::controllers::users_controller::UsersController;

fn main() {
    //Setup logging
    log4rs::init_file("config/log.toml", Default::default()).unwrap();
    info!("Initializing API");

    //Load Configuration
    let configuration = Config::new();

    //Initialize Database Connection
    let db = match DB::new(configuration){
        Ok(db) => db,
        Err(e) => {
            //Cannot create database connection
            panic!("{}", e);
        }
    };

    //Initialize DAO Manager
    let dao_manager = DAOManager::new(db);

    //Initialize Controllers
    let test_controller = TestController::new(dao_manager.clone());
    let users_controller = UsersController::new(dao_manager.clone());

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/", middleware! { |request, mut response|
        info!("API Endpoint: /");
        response.set(MediaType::Json);
        format!("{{\"status\":\"success\", \"msg\":\"Welcome to Money Map!\"}}")
    });

    router.get("/getDB", middleware! { |request, mut response|
        info!("API Endpoint: GET /getDB");
        response.set(MediaType::Json);
        /*match db.get_count(){
            Ok(count) => format!("{{\"status\":\"success\", \"msg\":\"Database Name: {}\"}}", count),
            Err(e) => {
                format!("{{\"status\":\"error\", \"msg\":\"{}\"}}", e)
            }
        }*/
    });

    //Test Actions
    router.get("/test/retrieve", middleware! { |request, mut response|
        info!("API Endpoint: POST /test/retrieve");
        response.set(MediaType::Json);
        let result = &test_controller.retrieve();
        JsonEncoder::encode(result)
    });

    /*router.post("/test/save", middleware! { |request, mut response|
        info!("API Endpoint: POST /test/save");
        response.set(MediaType::Json);
        let result = &test_controller.save(request);
        JsonEncoder::encode(result)
    });*/

    //Users Actions
    router.get("/users", middleware! { |request, mut response|
        info!("API Endpoint: GET /users");
        response.set(MediaType::Json);
        //let user = Controllers::users::PubUser::new("John".to_string(), "Smith".to_string(), "test@test.com".to_string());
        format!("{{\"status\":\"success\"}}")
    });

    router.post("/users", middleware! { |request, mut response|
        info!("API Endpoint: POST /users");
        response.set(MediaType::Json);
        match users_controller.create(request){
            Ok(response) => format!(r#"{{"status":"success", "data":{}}}"#, response),
            Err(e) =>format!(r#"{{"status":"error", "msg":"{}"}}"#, e)
        }
    });
    //router.post("/users", Resources::users::page);

    //server.utilize(authenticator);
    server.utilize(router);

    server.listen("0.0.0.0:6767");
}
