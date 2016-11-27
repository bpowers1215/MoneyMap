// src/dao/account_statement_dao.rs

/// Account Statement DAO
/// Handle all database interaction for Account Statements

// Import
extern crate mongodb;

// Import Modules
// Common Utilities
use ::bson::{Bson, Document};
use ::bson::oid::ObjectId;
use ::chrono::{Local};
use ::mongodb::coll::options::FindOptions;
use ::mongodb::db::ThreadedDatabase;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
// Models
use ::models::account_statement_model::{AccountStatementModel};

// Constants
static MONEY_MAP_COLLECTION: &'static str = "money_maps";

/// Account DAO
pub struct AccountStatementDAO{
    db: mongodb::db::Database
}

// Account Statement DAO Methods
impl AccountStatementDAO{
    /// Create AccountStatementDAO
    ///
    /// # Arguments
    /// db - mongodb::db::Database Cloned database connection
    ///
    /// # Returns
    /// `AccountStatementDAO`
    pub fn new(db: mongodb::db::Database) -> AccountStatementDAO{
        AccountStatementDAO{
            db: db
        }
    }
}
