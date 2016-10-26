// src/dao/account_dao.rs

/// Account DAO
/// Handle all database interaction for Accounts

// Import
extern crate mongodb;

// Import Modules
// Common Utilities
use ::bson::{Bson, Document};
use ::bson::oid::ObjectId;
use chrono::{UTC, Local, DateTime};
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

    /// Create Account
    /// Save account in specified money map
    ///
    /// # Arguments
    /// self
    /// mm_id - &str - Money Map ID
    /// account - &AccountModel The account
    ///
    /// # Returns
    /// `MMResult<AccountModel>`
    pub fn create(self, mm_id: ObjectId, account: &AccountModel) -> MMResult<AccountModel>{
        let coll = self.db.collection(MONEY_MAP_COLLECTION);

        let filter = doc! {
            "_id" => ( mm_id )
        };

        // Build `$push` document to update money map and add account
        let id = ObjectId::new().unwrap();
        let timestamp = Local::now().timestamp();
        let update_doc = doc! {
            "$push" => {
                "accounts" => {
                    "_id" => (Bson::ObjectId(id.clone())),
                    "name" => (match account.get_name(){Some(val) => val, None => "".to_string()}),
                    "account_type" => (match account.get_account_type(){Some(val) => val, None => "".to_string()}),
                    "created" => (Bson::TimeStamp(timestamp.clone()))
                }
            }
        };

        // Update the money map and add account
        match coll.update_one(filter.clone(), update_doc.clone(), None){
            Ok(result) => {
                if result.acknowledged && result.modified_count > 0 {
                    let mut new_account = account.clone();
                    new_account.set_id(Some(id));
                    new_account.set_created(Some(timestamp));
                    Ok(new_account)
                }else{
                    Err(MMError::new("Unable to add account to money map.", MMErrorKind::DAO))
                }
            },
            Err(e) => {
                error!("{}", e);
                Err(MMError::new("Failed to add account to money map.", MMErrorKind::DAO))
            }
        }
    }// end create
}
