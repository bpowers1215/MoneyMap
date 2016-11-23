// src/dao/account_dao.rs

/// Account DAO
/// Handle all database interaction for Accounts

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

    /// Find All Money Map accounts for filter
    ///
    /// # Arguments
    /// self
    /// user_id - ObjectId User ID
    /// mm_id - ObjectId Money Map ID
    ///
    /// # Returns
    /// `Option<Vec<AccountModel>>`
    pub fn find(self, user_id: ObjectId, mm_id: ObjectId) -> Option<Vec<AccountModel>>{
        let coll = self.db.collection(MONEY_MAP_COLLECTION);
        let mut accounts = Vec::new();

        let pipeline = vec![
            doc!{
                // Filter Money Map
                "$match" => {
                    "_id" => mm_id,
                    "users.user_id" => user_id,
                    "deleted" => {
                        "$ne" => true
                    }
                }
            },
            doc!{
                "$unwind" => "$accounts"
            },
            doc!{
                // Exclude deleted accounts
                "$match" => {
                    "accounts.deleted" => {
                        "$ne" => true
                    }
                }
            },
            doc!{
                "$project" => {
                    "_id" => "$accounts._id",
                    "name" => "$accounts.name",
                    "account_type" => "$accounts.account_type",
                    "created" => "$accounts.created"
                }
            }
        ];

        match coll.aggregate(pipeline, None){
            Ok(cursor) => {
                for result in cursor {
                    if let Ok(acc_doc) = result {
                        accounts.push(document_to_model(&acc_doc));
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

    /// Find One Account
    ///
    /// # Arguments
    /// self
    /// filter - Option<Document> The find filter
    ///
    /// # Returns
    /// `Option<AccountModel>`
    pub fn find_one(&self, filter: Option<Document>) -> Option<AccountModel>{
        let coll = self.db.collection(MONEY_MAP_COLLECTION);

        let mut find_options = FindOptions::new();
        find_options.projection = Some(doc!{
            "_id" => 0,
            "accounts.$" => 1
        });

        match coll.find_one(filter, Some(find_options)){
            Ok(result) => {
                match result{
                    Some(document) => {
                        if let Some(raw_accounts) = document.get("accounts"){
                            if let &Bson::Array(ref raw_accounts_arr) = raw_accounts{
                                if raw_accounts_arr.len() == 1 {
                                    if let Bson::Document(ref acc_doc) = raw_accounts_arr[0]{
                                        return Some(document_to_model(acc_doc));
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
        None
    }// end find_one

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

    /// Update an existing acount
    ///
    /// # Arguments
    /// self
    /// mm_id ObjectId The Money Map ID
    /// &account - AccountModel
    ///
    /// # Returns
    /// `MMResult<AccountModel>` The updated money map if successful, None otherwise
    pub fn update(&self, mm_id: ObjectId, account: &AccountModel) -> MMResult<AccountModel>{
        let coll = self.db.collection(MONEY_MAP_COLLECTION);

        let filter = doc! {
            //Money Map Information
            "_id" => mm_id,
            "deleted" => {
                "$ne" => true
            },
            //Account Information
            "accounts" => {
                "$elemMatch" => {
                    "_id" => ( account.get_id().unwrap() ),
                    "deleted" => {
                        "$ne" => true
                    }
                }
            }
        };

        // Build `$set` document to update document
        let mut set_doc = doc!{};
        let mut update = false;
        if let Some(name) = account.get_name(){
            update = true;
            set_doc.insert_bson("accounts.$.name".to_string(), Bson::String(name));
        }
        if let Some(account_type) = account.get_account_type(){
            update = true;
            set_doc.insert_bson("accounts.$.account_type".to_string(), Bson::String(account_type));
        }
        let update_doc = if update {
            doc! {"$set" => set_doc}
        }else{
            // No updates to account, return existing account details if found
            return match self.find_one(Some(filter)){
                Some(result) => Ok(result),
                None => Err(MMError::new("Unable to find account", MMErrorKind::DAO))
            };
        };

        // Update the money map
        match coll.update_one(filter.clone(), update_doc.clone(), None){
            Ok(result) => {
                match result.write_exception{
                    None => {
                        if result.matched_count > 0{
                            // Account found and updated
                            Ok(self.find_one(Some(filter)).unwrap())
                        }else{
                            Err(MMError::new("Unable to find account", MMErrorKind::DAO))
                        }
                    },
                    Some(_) => {
                        Err(MMError::new("Unable to save account", MMErrorKind::DAO))
                    }
                }

            },
            Err(e) => {
                error!("{}", e);
                Err(MMError::new("Failed to update money map.", MMErrorKind::DAO))
            }
        }
    }// end update

    /// Delete an Account
    /// Only allow deleting an account owned by the current user
    ///
    /// # Arguments
    /// self
    /// user_id - ObjectId User ID
    /// mm_id - ObjectId Money Map ID
    /// acc_id - ObjectId Money Map ID
    ///
    /// # Returns
    /// `MMResult<()>`
    pub fn delete(self, user_id: ObjectId, mm_id: ObjectId, acc_id: ObjectId) -> MMResult<mongodb::coll::results::UpdateResult>{
        let coll = self.db.collection(MONEY_MAP_COLLECTION);

        let filter = doc! {
            //Money Map Information
            "_id" => mm_id,
            "deleted" => {
                "$ne" => true
            },
            "users" => {
                "$elemMatch" => {
                    "user_id" => user_id
                }
            },
            "accounts" => {
                "$elemMatch" => {
                    "deleted" => {
                        "$ne" => true
                    }
                }
            },
            //Account Information
            "accounts._id" => acc_id
        };

        let update_doc = doc!{
            "$set" => {
                "accounts.$.deleted" => true
            }
        };

        // Soft delete money map
        match coll.update_one(filter.clone(), update_doc.clone(), None){
            Ok(result) => Ok(result),
            Err(e) => {
                error!("{}", e);
                Err(MMError::new("Failed to delete account.", MMErrorKind::DAO))
            }
        }
    }// end delete
}

/// Create AccountModel from Document
///
/// # Arguments
/// self
/// doc - Document
///
/// # Returns
/// `AccountModel`
fn document_to_model(doc: &Document) -> AccountModel{
    AccountModel{
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
            Some(&Bson::TimeStamp(ref timestamp)) => Some(timestamp.clone()),
            _ => None
        }
    }
}
