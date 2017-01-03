// src/dao/transaction_dao.rs

/// Transaction DAO
/// Handle all database interaction for Transaction collection

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
use ::models::transaction_model::{TransactionModel};

// Constants
static TRANSACTION_COLLECTION: &'static str = "transactions";

/// Transaction DAO
pub struct TransactionDAO{
    db: mongodb::db::Database
}

// Transaction DAO Methods
impl TransactionDAO{
    /// Create TransactionDAO
    ///
    /// # Arguments
    /// db - mongodb::db::Database Cloned database connection
    ///
    /// # Returns
    /// `TransactionDAO`
    pub fn new(db: mongodb::db::Database) -> TransactionDAO{
        TransactionDAO{
            db: db
        }
    }
}
