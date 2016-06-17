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

/// User
#[derive(RustcDecodable, RustcEncodable)]
pub struct User {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>
}

/// Edit User
#[derive(RustcDecodable, RustcEncodable)]
pub struct EditUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub confirm_password: Option<String>
}

// User Methods
impl User{

}

// EditUser Methods
impl EditUser{
    
    /// Save User
    ///
    /// # Returns
    /// 'MMResult<User>' - the saved user
    pub fn save(self) -> MMResult<User>{
        //validate user
        
        //save user to database
        
        //remove sensitive data and return new User struct
        Ok(User{
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email
        })
    }

}