// src/dao/account_dao.rs

/// Account DAO
/// Handle all database interaction for Accounts

// Import
extern crate mongodb;

// Import Modules
// Common Utilities
use ::bson::{Bson, Document};
use ::bson::oid::ObjectId;
use chrono::{UTC, Local, DateTime, TimeZone};
use ::mongodb::coll::options::FindOptions;
use ::mongodb::db::ThreadedDatabase;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
// Models
use ::models::account_model::{AccountModel, OutAccountModel};

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

    /// Find All Money Map accounts for filter
    ///
    /// # Arguments
    /// self
    /// filter - Option<Document> The find filter
    ///
    /// # Returns
    /// `Option<Vec<OutAccountModel>>`
    pub fn find(self, filter: Option<Document>) -> Option<Vec<OutAccountModel>>{
        let coll = self.db.collection(MONEY_MAP_COLLECTION);
        let mut accounts = Vec::new();

        let mut find_options = FindOptions::new();
        find_options.projection = Some(doc!{
            "_id" => 0,
            "name" => 0,
            "users" => 0,
            "deleted" => 0
        });

        match coll.find_one(filter, Some(find_options)){
            Ok(result) => {
                match result{
                    Some(document) => {
                        if let Some(raw_accounts) = document.get("accounts"){
                            if let &Bson::Array(ref raw_accounts_arr) = raw_accounts{
                                for acc in raw_accounts_arr {
                                    if let &Bson::Document(ref acc_doc) = acc{
                                        accounts.push(document_to_model(acc_doc));
                                    }
                                }
                            }
                        }
                    },
                    None => {
                        //Could not find money map for user
                        return None
                    }
                }
            },
            Err(e) => {
                error!("Find All accounts failed: {}", e);
                return None;
            }
        }
        Some(accounts)
    }// end find

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

/// Create OutAccountModel from Document
///
/// # Arguments
/// self
/// doc - Document
///
/// # Returns
/// `OutAccountModel`
fn document_to_model(doc: &Document) -> OutAccountModel{
    OutAccountModel{
        id: match doc.get("_id"){
            Some(obj_id) => match obj_id{ &Bson::ObjectId(ref id) => Some(id.clone()), _ => None},
            _ => None
        },
        name: match doc.get("name"){
            Some(&Bson::String(ref name)) => Some(name.clone()),
            _ => None
        },
        account_type: match doc.get("account_type"){
            Some(&Bson::String(ref account_type)) => Some(account_type.clone()),
            _ => None
        },
        created: match doc.get("created"){
            Some(&Bson::TimeStamp(ref timestamp)) => {
                Some(Local.timestamp(timestamp.clone(), 0).to_rfc2822())
            },
            _ => None
        }
    }
}
