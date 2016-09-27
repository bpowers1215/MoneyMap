// src/dao/money_map_dao.rs

/// Money Map DAO
/// Handle all database interaction for Money Map collection

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
use ::models::money_map_model::{MoneyMapModel};

/// Money Map DAO
pub struct MoneyMapDAO{
    db: mongodb::db::Database
}

// Money Map DAO Methods
impl MoneyMapDAO{
    /// Create MoneyMapDAO
    ///
    /// # Arguments
    /// db - mongodb::db::Database Cloned database connection
    ///
    /// # Returns
    /// `MoneyMapDAO`
    pub fn new(db: mongodb::db::Database) -> MoneyMapDAO{
        MoneyMapDAO{
            db: db
        }
    }

    /// Create Money Map
    /// Save new money mmap to the money maps collection
    ///
    /// # Arguments
    /// self
    /// &user - models::money_map_model::MoneyMapModel The Money Map
    ///
    /// # Returns
    /// `MMResult<()>`
    pub fn create(self, money_map: &MoneyMapModel) -> MMResult<mongodb::coll::results::InsertOneResult>{
        let coll = self.db.collection("money_maps");

        let doc = doc! {
            "name" => (match money_map.get_name(){Some(val) => val, None => "".to_string()})
        };

        // Insert document into `money_maps` collection
        match coll.insert_one(doc.clone(), None){
            Ok(result) => Ok(result),
            Err(e) => {
                warn!("{}", e);
                Err(MMError::new("Failed to insert money_map", MMErrorKind::DAO))
            }
        }
    }// end create
}
