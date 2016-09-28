// src/dao/money_map_dao.rs

/// Money Map DAO
/// Handle all database interaction for Money Map collection

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

// Constants
static money_map_collection: &'static str = "money_maps";

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

    /// Find All Money Maps belonging to a user
    ///
    /// # Arguments
    /// self
    ///
    /// # Returns
    /// `Vec<MoneyMapModel>`
    pub fn find_all(self) -> Vec<MoneyMapModel>{
        let coll = self.db.collection(money_map_collection);
        let mut money_maps = Vec::new();

        // Set Find Options and retrieve cursor
        let mut find_options = FindOptions::new();

        match coll.find(None, Some(find_options)){
            Ok(cursor) => {
                for result in cursor {
                    if let Ok(item) = result {
                        let money_map = MoneyMapModel{
                            id: match item.get("_id"){
                                Some(obj_id) => match obj_id{ &Bson::ObjectId(ref id) => Some(id.clone()), _ => None},
                                _ => None
                            },
                            name: match item.get("name"){
                                Some(&Bson::String(ref name)) => Some(name.clone()),
                                _ => None
                            }
                        };
                        money_maps.push(money_map);
                    }
                }
            },
            Err(e) => {
                error!("Find All money_maps failed: {}", e)
            }
        }
        money_maps
    }// end find_all

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
        let coll = self.db.collection(money_map_collection);

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
