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
static MONEY_MAP_COLLECTION: &'static str = "money_maps";

/// Transaction DAO
pub struct TransactionDAO{
    db: mongodb::db::Database
}

// Transaction DAO Methods
impl TransactionDAO{
    /// Create TransactionDAO
    ///
    /// # Arguments
    /// `db` - Cloned database connection
    ///
    /// # Returns
    /// `TransactionDAO`
    pub fn new(db: mongodb::db::Database) -> TransactionDAO{
        TransactionDAO{
            db: db
        }
    }

    /// Check if an account is valid to receive transactions
    /// Factors:
    ///     Valid/active Money Map
    ///     Valid/active Account
    ///     User access to money Map
    ///
    /// # Arguments
    /// `self`
    /// `user_id` - ObjectId User ID
    /// `money_map_id` - ObjectId User ID
    /// `account_id` - ObjectId User ID
    ///
    /// # Returns
    /// `bool` True if valid account, false otherwise
    pub fn is_valid_account(&self, user_id: ObjectId, money_map_id: ObjectId, account_id: ObjectId) -> bool{
        let coll = self.db.collection(MONEY_MAP_COLLECTION);

        let filter = doc!{
            "_id" => money_map_id,
            "users.user_id" => user_id,
            "accounts._id" => account_id
        };

        match coll.find_one(Some(filter), None){
            Ok(result) => {
                match result{
                    Some(document) => {
                        true
                    },
                    None => {
                        //Could not find money map for user/account
                        false
                    }
                }
            },
            Err(e) => {
                error!("Find account failed: {}", e);
                false
            }
        }
    }
}
