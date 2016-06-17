// src/resources/user.rs

/// User Model

//Import Modules
use ::rustc_serialize::json;
use ::mongodb::{Client, ThreadedClient};
use ::mongodb::db::ThreadedDatabase;
use ::mongodb::error::Result as MongoResult;
use ::common::config::Config;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::common::database::DB;

// Nickel
//use nickel::{JsonBody, Request, Response};
use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, MiddlewareResult, MediaType};

/// Represent a User - visible data
pub struct PubUser{
    first_name: String,
    last_name: String,
    email: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct NewUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub confirm_password: Option<String>
}

impl PubUser{

    /// Create a PubUser
    ///
    /// # Returns
    /// `PubUser`
    pub fn new(first_name: String, last_name: String, email: String) -> PubUser{
        PubUser{
            first_name: first_name,
            last_name: last_name,
            email: email
        }
    }

    /// Get First Name
    ///
    /// # Returns
    /// `String` - first name
    pub fn get_first_name(self) -> String{
        self.first_name
    }

    /// Get Last Name
    ///
    /// # Returns
    /// `String` - last name
    pub fn get_last_name(self) -> String{
        self.last_name
    }

    /// Get Name
    ///
    /// # Returns
    /// `String` - full name
    pub fn get_name(self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

}

/// Create New User
///
/// # Arguments
///
/// # Returns
/// `MMResult<()>`
pub fn create_user(req: &mut Request) -> String{
    info!("Create User");
    let new_user = req.json_as::<NewUser>().unwrap();
    
    // Serialize using `json::encode`
    let encoded = json::encode(&new_user).unwrap();
    match new_user.first_name{
        Some(name) => format!(r#"{{"status":"success", "data":{}}}"#, encoded),
        None => format!(r#"{{"status":"success", "msg":"no first name"}}"#)
    }
}

/*pub fn page<'mw, 'conn> (req: &mut Request<'mw, 'conn>, res: &mut Response<'mw>) -> String{
    info!("PAGE ACCESSED!");
    let new_user = req.json_as::<NewUser>().unwrap();
    debug!("NAME: {}", new_user.first_name);
    res.set(MediaType::Json);
    format!("{{\"status\":\"success\", \"msg\":\"Welcome to Money Map!\"}}")
}*/
