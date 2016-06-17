// src/controllers/users.rs

/// Users Controller

//Import Modules
use ::rustc_serialize::json;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::common::database::DB;

//Models
use ::models::user::{User, EditUser};

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
    info!("Create New User");
    let new_user = req.json_as::<EditUser>().unwrap();
    
    //Save User
    let user = new_user.save();
    
    //Return Saved User
    match user{
        Ok(u) => {
            let response = json::encode(&u).unwrap();
            format!(r#"{{"status":"success", "data":{{"user":{}}}}}"#, response)
        },
        Err(_) => {
            format!(r#"{{"status":"error", "msg":"Could not create user""#)
        }
    }
    
}