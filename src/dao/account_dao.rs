// src/dao/account_dao.rs

/// Account DAO
/// Handle all database interaction for Accounts

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
use ::models::account_model::{AccountModel};

// Constants
static MONEY_MAP_COLLECTION: &'static str = "money_maps";

/// Account DAO
pub struct AccountDAO{
    db: mongodb::db::Database
}

// Account DAO Methods
impl AccountDAO{
    /// Create AccountDAO
    ///
    /// # Arguments
    /// db - mongodb::db::Database Cloned database connection
    ///
    /// # Returns
    /// `AccountDAO`
    pub fn new(db: mongodb::db::Database) -> AccountDAO{
        AccountDAO{
            db: db
        }
    }
}
