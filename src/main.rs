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
//extern crate money-map;

//Logging
#[macro_use]
extern crate log;
extern crate log4rs;

// Nickel
use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};
use nickel::status::StatusCode::{self, Forbidden};

// MongoDB
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::error::Result as MongoResult;

// bson
use bson::{Bson, Document};
use bson::oid::ObjectId;

// rustc_serialize
use rustc_serialize::json::{Json, ToJson};
use rustc_serialize::base64;
use rustc_serialize::base64::{FromBase64};

// hyper
use hyper::header;
use hyper::header::{Authorization, Bearer};
use hyper::method::Method;

// jwt
use std::default::Default;
use crypto::sha2::Sha256;
use jwt::{
    Header,
    Registered,
    Token,
};

use chrono::*;

//Constants
const MONGO_DB_PORT: u16 = 27017;
static MONGO_DB_HOST: &'static str = "mongoDB";
static MONGO_DB_NAME: &'static str = "todo";
static MONGO_DB_USER: &'static str = "todo";
static MONGO_DB_PW: &'static str = "SECRET_PASSWORD";

static AUTH_SECRET: &'static str = "dijf8934(Gq0h98)!43nvaHGs";

//Structs
#[derive(RustcDecodable, RustcEncodable)]
struct User{
    first_name: String,
    last_name: String,
    email: String,
    password: String
}
#[derive(RustcDecodable, RustcEncodable)]
struct UserLogin{
    email: String,
    password: String
}

/*
*   Get an authenticated DB connection
*   Return the DB
*/
fn get_db_connection() -> mongodb::db::Database{
    // Connect to the database
    let client = Client::connect(MONGO_DB_HOST, MONGO_DB_PORT)
        .ok().expect("Error establishing database connection.");

    // Get admin DB
    let db = client.db(MONGO_DB_NAME);

    // Authenticate admin user for admin DB
    db.auth(MONGO_DB_USER, MONGO_DB_PW)
        .ok().expect("Failed to authorize user 'todo'.");
    db
}

fn get_data_string(result: MongoResult<Document>) -> Result<Json, String> {
    match result {
        Ok(doc) => Ok(Bson::Document(doc).to_json()),
        Err(e) => Err(format!("{}", e))
    }
}

fn main() {
    log4rs::init_file("log.toml", Default::default()).unwrap();
    info!("starting up");

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/", middleware! { |request, mut response|
        info!("Welcome");
        response.set(MediaType::Json);
        let result = "{\"status\":\"success\", \"msg\":\"Welcome to Money Map\"}";
        format!("{}", result)
    });

    //server.utilize(authenticator);
    server.utilize(router);

    server.listen("0.0.0.0:6767");
}
