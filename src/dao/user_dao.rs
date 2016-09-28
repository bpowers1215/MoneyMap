// src/dao/user_dao.rs

/// User DAO
/// Handle all database interaction for Users collection

//import
extern crate mongodb;

// Import Modules
// Common Utilities
use ::bson::{Bson, Document};
use ::bson::oid::ObjectId;
use ::mongodb::coll::options::FindOptions;
use ::mongodb::db::ThreadedDatabase;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
//Models
use ::models::user_model::{UserModel, OutUserModel};

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

    /// Find All Users
    ///
    /// # Arguments
    /// self
    ///
    /// # Returns
    /// `Vec<OutUserModel>`
    pub fn find(self) -> Vec<OutUserModel>{
        let coll = self.db.collection("users");
        let mut users = Vec::new();

        //Set Find Options and retrieve cursor
        let mut find_options = FindOptions::new();
        find_options.projection = Some(doc!{
            "password" => 0//exclude password
        });

        match coll.find(None, Some(find_options)){
            Ok(cursor) => {
                for result in cursor {
                    if let Ok(item) = result {
                        let user = OutUserModel{
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
                            }
                        };
                        users.push(user);
                    }
                }
            },
            Err(e) => {
                error!("Find All Users failed: {}", e)
            }
        }
        users
    }// end find

    /// Find a single User
    ///
    /// # Arguments
    /// self
    /// filter - Option<Document> The find filter
    /// options - Option<FindOptions> The find options
    ///
    /// # Returns
    /// `Option<UserModel>` Some UserModel if found, None otherwise
    pub fn find_one(self, filter: Option<Document>, options: Option<FindOptions>) -> Option<UserModel>{
        let coll = self.db.collection("users");

        match coll.find_one(filter, options){
            Ok(result) => {
                if let Some(document) = result{
                        Some(UserModel{
                            id: match document.get("_id"){
                                Some(obj_id) => match obj_id{ &Bson::ObjectId(ref id) => Some(id.clone()), _ => None},
                                _ => None
                            },
                            first_name: match document.get("first_name"){
                                Some(&Bson::String(ref first_name)) => Some(first_name.clone()),
                                _ => None
                            },
                            last_name: match document.get("last_name"){
                                Some(&Bson::String(ref last_name)) => Some(last_name.clone()),
                                _ => None
                            },
                            email: match document.get("email"){
                                Some(&Bson::String(ref email)) => Some(email.clone()),
                                _ => None
                            },
                            password: match document.get("password"){
                                Some(&Bson::String(ref password)) => Some(password.clone()),
                                _ => None
                            }
                        })
                }else{
                    None
                }
            },
            Err(e) => {
                error!("Find User failed: {}", e);
                None
            }
        }
    }// end find_one

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
            Err(e) => {
                warn!("{}", e);
                Err(MMError::new("Failed to insert user", MMErrorKind::DAO))
            }
        }
    }// end create

    /// Save an existing User
    ///
    /// # Arguments
    /// self
    /// user_id - String User identifier
    /// &user - models::user_model::UserModel The user
    ///
    /// # Returns
    /// `MMResult<()>`
    pub fn update(self, user_id: String, user: &UserModel) -> MMResult<mongodb::coll::results::UpdateResult>{
        let coll = self.db.collection("users");

        match ObjectId::with_string(user_id.as_str()){
            Ok(id) => {
                let filter = doc! {
                    "_id" => id
                };

                // Build `$set` document to update document
                let mut set_doc = doc!{};
                if let Some(first_name) = user.get_first_name(){
                    set_doc.insert_bson("first_name".to_string(), Bson::String(first_name));
                }
                if let Some(last_name) = user.get_last_name(){
                    set_doc.insert_bson("last_name".to_string(), Bson::String(last_name));
                }
                if let Some(password) = user.get_password(){
                    set_doc.insert_bson("password".to_string(), Bson::String(password));
                }
                let update_doc = doc! {"$set" => set_doc};

                // Update the user
                match coll.update_one(filter.clone(), update_doc.clone(), None){
                    Ok(result) => Ok(result),
                    Err(e) => {
                        error!("{}", e);
                        Err(MMError::new("Failed to update user.", MMErrorKind::DAO))
                    }
                }
            },
            Err(e) => {
                error!("{}", e);
                Err(MMError::new("Failed to update user.", MMErrorKind::DAO))
            }
        }
    }// end update
}
