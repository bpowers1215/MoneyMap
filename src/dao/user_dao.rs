// src/dao/user_dao.rs

/// User DAO
/// Handle all database interaction for Users collection

//import
extern crate mongodb;

//Import Modules
use ::mongodb::{Client, ThreadedClient};
use ::mongodb::db::ThreadedDatabase;
use ::mongodb::error::Result as MongoResult;
use ::common::config::Config;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};

/// User DAO
pub struct UserDAO{
    db: mongodb::db::Database
}

// User DAO Methods
impl UserDAO{
    /// Create UserDAO
    ///
    /// # Arguments
    /// db - mongodb::db::Database Cloned database connection
    ///
    /// # Returns
    /// `UserDAO`
    pub fn new(db: mongodb::db::Database) -> UserDAO{
        UserDAO{
            db: db
        }
    }
    
    pub fn test(self){
        info!("We have a UserDAO!!!");
    }
}
