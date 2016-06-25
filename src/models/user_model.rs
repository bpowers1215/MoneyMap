// src/resources/user_model.rs

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
pub struct UserModel {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>
}

// User Model Methods
impl UserModel{

    /// Create User
    ///
    /// # Returns
    /// 'MMResult<UserModel>' - the saved user
    pub fn create(self) -> MMResult<UserModel>{
        //validate user

        //save user to database

        //remove sensitive data and return new User struct
        Ok(UserModel{
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            password: None
        })
    }//end save

}
