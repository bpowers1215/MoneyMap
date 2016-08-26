// src/dao/user_dao.rs

/// User DAO
/// Handle all database interaction for Users collection

//import
extern crate mongodb;

//Import Modules
use ::bson::{Bson};
use ::bson::oid::ObjectId;
use ::mongodb::coll::options::FindOptions;
use ::mongodb::{Client, ThreadedClient};
use ::mongodb::db::ThreadedDatabase;
use ::mongodb::error::Result as MongoResult;
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

    /// Fetch All Users
    ///
    /// # Arguments
    /// self
    ///
    /// # Returns
    /// `Vec<UserModel>`
    pub fn fetch_all(self) -> Vec<UserModel>{
        let coll = self.db.collection("users");
        
        //Set Find Options and retrieve cursor
        let mut find_options = FindOptions::new();
        find_options.projection = Some(doc!{
            "password" => 0//exclude password
        });
        let mut cursor = coll.find(None, Some(find_options)).unwrap();
        
        let mut users = Vec::new();
        
        for result in cursor {
            if let Ok(item) = result {
                let user = UserModel{
                    id: match item.get("_id"){ 
                        Some(obj_id) => match obj_id{ &Bson::ObjectId(ref id) => Some(id.clone()), _ => None},
                        _ => None
                    },
                    first_name: match item.get("first_name"){ 
                        Some(&Bson::String(ref first_name)) => Some(first_name.clone()),
                        _ => None
                    },
                    last_name: match item.get("last_name"){ 
                        Some(&Bson::String(ref last_name)) => Some(last_name.clone()),
                        _ => None
                    },
                    email: match item.get("email"){ 
                        Some(&Bson::String(ref email)) => Some(email.clone()),
                        _ => None
                    },
                    password: None
                };
                users.push(user);
            }
        }
        users
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
    pub fn create(self, user: &UserModel) -> MMResult<mongodb::coll::results::InsertOneResult>{
        let coll = self.db.collection("users");

        let doc = doc! {
            "first_name" => (match user.get_first_name(){Some(val) => val, None => "".to_string()}),
            "last_name" => (match user.get_last_name(){Some(val) => val, None => "".to_string()}),
            "email" => (match user.get_email(){Some(val) => val, None => "".to_string()}),
            "password" => (match user.get_password(){Some(val) => val, None => "".to_string()})
        };

        // Insert document into `users` collection
        match coll.insert_one(doc.clone(), None){
            Ok(result) => Ok(result),
            Err(_) => Err(MMError::new("Failed to insert user".to_string(), MMErrorKind::DAO))
        }
    }
}
