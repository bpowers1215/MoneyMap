// src/controllers/users.rs

/// Users Controller

//Import Modules
use ::rustc_serialize::json;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::common::database::DB;

//Models
use ::models::user::{NewUser, PubUser};

// Nickel
//use nickel::{JsonBody, Request, Response};
use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};


/// Create New User
///
/// # Arguments
/// req - nickel::Request
///
/// # Returns
/// `String` - JSON String response
pub fn create_user(req: &mut Request) -> String{
    info!("Create User");
    let new_user = req.json_as::<NewUser>().unwrap();
    
    // Serialize using `json::encode`
    let encoded = json::encode(&new_user).unwrap();
    match new_user.first_name{
        Some(name) => format!(r#"{{"status":"success", "data":{{"user":{}}}}}"#, encoded),
        None => format!(r#"{{"status":"success", "msg":"no first name"}}"#)
    }
}