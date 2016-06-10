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

use money_map::common::database::DB as DB;
use money_map::common::config::Config as Config;

fn main() {
    //Setup logging
    log4rs::init_file("config/log.toml", Default::default()).unwrap();
    info!("Initializing API");

    //Load Configuration
    let configuration = Config::new();

    //Get database
    let db = match DB::new(configuration){
        Ok(db) => db,
        Err(e) => {
            //Cannot create database connection
            panic!("{}", e);
        }
    };

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/", middleware! { |request, mut response|
        info!("API Endpoint: /");
        response.set(MediaType::Json);
        format!("{{\"status\":\"success\", \"msg\":\"Welcome to Money Map!\"}}")
    });

    router.get("/getDB", middleware! { |request, mut response|
        info!("API Endpoint: /getDB");
        response.set(MediaType::Json);
        match db.get_count(){
            Ok(count) => format!("{{\"status\":\"success\", \"msg\":\"Database Name: {}\"}}", count),
            Err(e) => {
                format!("{{\"status\":\"error\", \"msg\":\"{}\"}}", e)
            }
        }

    });

    //server.utilize(authenticator);
    server.utilize(router);

    server.listen("0.0.0.0:6767");
}
