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

//Models
use ::models::user_model::{UserModel};

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

    /// Create User
    /// Save new user to the users collection
    ///
    /// # Arguments
    /// self
    /// &user - models::user_model::UserModel The user
    ///
    /// # Returns
    /// `MMResult<()>`
    pub fn create(self, user: &UserModel) -> MMResult<()>{
        let coll = self.db.collection("users");

        let doc = doc! {
            "first_name" => (match user.get_first_name(){Some(val) => val, None => "".to_string()}),
            "last_name" => (match user.get_last_name(){Some(val) => val, None => "".to_string()}),
            "email" => (match user.get_email(){Some(val) => val, None => "".to_string()}),
            "password" => (match user.get_password(){Some(val) => val, None => "".to_string()})
        };

        //TODO: Handle error on insert
        // Insert document into `users` collection
        coll.insert_one(doc.clone(), None)
            .ok().expect("Failed to insert user.");

        //TODO: Return proper Results
        Ok(())
    }
}
