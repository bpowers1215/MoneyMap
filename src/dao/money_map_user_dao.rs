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
}
