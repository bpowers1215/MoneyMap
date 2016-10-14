// src/dao/money_map_user_dao.rs

/// Money Map User DAO
/// Handle all database interaction for Money Map Users

// Import
extern crate mongodb;

// Import Modules
// Common Utilities
use ::bson::{Bson, Document};
use ::bson::oid::ObjectId;
use ::mongodb::coll::options::FindOptions;
use ::mongodb::db::ThreadedDatabase;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
// Models
use ::models::money_map_model::{MoneyMapModel};
use ::models::money_map_user_model::{MoneyMapUserModel};
use ::models::user_model::{OutUserModel};

// Constants
static MONEY_MAP_COLLECTION: &'static str = "money_maps";

/// Money Map User DAO
pub struct MoneyMapUserDAO{
    db: mongodb::db::Database
}

// Money Map User DAO Methods
impl MoneyMapUserDAO{
    /// Create MoneyMapDAO
    ///
    /// # Arguments
    /// db - mongodb::db::Database Cloned database connection
    ///
    /// # Returns
    /// `MoneyMapDAO`
    pub fn new(db: mongodb::db::Database) -> MoneyMapUserDAO{
        MoneyMapUserDAO{
            db: db
        }
    }

    /// Add user to money map
    ///
    /// # Arguments
    /// self
    /// mm_id - ObjectId The Money Map ID
    /// user_id - ObjectId The User ID
    ///
    /// # Returns
    /// `MMResult<UpdateResult>`
    pub fn add_user(&self, mm_id: ObjectId, user_id: ObjectId) -> MMResult<mongodb::coll::results::UpdateResult>{
        let coll = self.db.collection(MONEY_MAP_COLLECTION);

        let filter = doc! {
            "_id" => ( mm_id )
        };

        // Build `$set` document to update document
        let update_doc = doc! {
            "$push" => {
                "users" => {
                    "user_id" => user_id,
                    "owner" => false
                }
            }
        };

        // Update the money map
        match coll.update_one(filter.clone(), update_doc.clone(), None){
            Ok(result) => {
                Ok(result)
            },
            Err(e) => {
                error!("{}", e);
                Err(MMError::new("Failed to update money map.", MMErrorKind::DAO))
            }
        }
    }// end add_user
}
