// src/models/users.rs

/// Users

//Import Modules
use ::mongodb::{Client, ThreadedClient};
use ::mongodb::db::ThreadedDatabase;
use ::mongodb::error::Result as MongoResult;
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
struct NewUser {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    password: Option<String>,
    confirm_password: Option<String>
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
