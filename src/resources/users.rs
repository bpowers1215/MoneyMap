// src/resources/users.rs

/// Users

//Import Crates
extern crate mongodb;

//Import Modules
use self::mongodb::{Client, ThreadedClient};
use self::mongodb::db::ThreadedDatabase;
use self::mongodb::error::Result as MongoResult;
use ::common::config::Config;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::common::database::DB;

/// Represent a User - visible data
pub struct PubUser{
    first_name: String,
    last_name: String,
    email: String
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
