// src/dao/account_statement_dao.rs

/// Account Statement DAO
/// Handle all database interaction for Account Statements

// Import
extern crate mongodb;

// Import Modules
// External
use ::bson::{Bson, Document};
use ::bson::oid::ObjectId;
use ::mongodb::db::ThreadedDatabase;
use ::chrono::{DateTime};
use ::chrono::Utc as UTC;
// Common Utilities
use ::common::utilities as Utilities;
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

    /// Find All Account Statement
    ///
    /// # Arguments
    /// `&self`
    /// `user_id` - ObjectId User ID
    /// `mm_id` - ObjectId Money Map ID
    /// `acc_id` - ObjectId Money Map ID
    /// `sort` - A Vector of SortParams
    /// `start_date` - Start DateTime (inclusive)
    /// `end_date` - End DateTime (exclusive)
    ///
    /// # Returns
    /// `Option<Vec<AccountModel>>`
    pub fn find(&self, user_id: ObjectId, mm_id: ObjectId, acc_id: ObjectId, sort: Vec<Utilities::url::SortParam>, start_date: Option<DateTime<UTC>>, end_date: Option<DateTime<UTC>>) -> Option<Vec<AccountStatementModel>>{
        let coll = self.db.collection(MONEY_MAP_COLLECTION);
        let mut accounts = Vec::new();

        let mut pipeline = vec![
            doc!{
                // Match Money Map
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
                // Match Account
                "$match" => {
                    "accounts._id" => acc_id
                }
            },
            doc!{
                "$unwind" => "$accounts.statements"
            },
            doc!{
                "$project" => {
                    "_id" => false,
                    "statement_date" => "$accounts.statements.statement_date",
                    "ending_balance" => "$accounts.statements.ending_balance"
                }
            }
        ];

        // Add Start Date match to pipeline
        if let Some(sd) = start_date{
            pipeline.push(
                doc!{
                    "$match" => {
                        "statement_date" => {
                            "$gte" => sd
                        }
                    }
                }
            );
        }

        // Add End Date match to pipeline
        if let Some(ed) = end_date{
            pipeline.push(
                doc!{
                    "$match" => {
                        "statement_date" => {
                            "$lt" => ed
                        }
                    }
                }
            );
        }

        // Add sort phase to pipeline
        if !sort.is_empty() {
            let mut sort_doc = doc!{};
            for ref x in sort.iter() {
                sort_doc.insert_bson(x.field.clone(), Bson::I32(x.direction));
            }
            pipeline.push(
                doc!{
                    "$sort" => sort_doc
                }
            );
        }

        match coll.aggregate(pipeline, None){
            Ok(cursor) => {
                for result in cursor {
                    if let Ok(acc_doc) = result {
                        accounts.push(document_to_model(&acc_doc));
                    }
                }
            },
            Err(e) => {
                error!("Find All account statements failed: {}", e);
                return None;
            }
        }
        Some(accounts)
    }// end find

    /// Create Account Statement
    ///
    /// # Arguments
    /// `&self`
    /// `statement` - &AccountStatementModel The transaction
    /// `mm_id` - ObjectId Money Map ID
    /// `acc_id` - ObjectId Account ID
    ///
    /// # Returns
    /// `MMResult<AccountStatementModel>`
    pub fn create(&self, statement: &AccountStatementModel, mm_id: ObjectId, acc_id: ObjectId) -> MMResult<AccountStatementModel>{
        let coll = self.db.collection(MONEY_MAP_COLLECTION);

        let filter = doc! {
            "_id" => ( mm_id ),
            "accounts" => {
                "$elemMatch" => {
                    "_id" => (acc_id)
                }
            }
        };
        let update_doc = doc! {
            "$addToSet" => {
                "accounts.$.statements" => {
                    "statement_date" => (statement.get_statement_date().unwrap()),
                    "ending_balance" => (match statement.get_ending_balance(){Some(val) => val, None => 0.0})
                }
            }
        };

        // Insert document into `transactions` collection
        match coll.update_one(filter.clone(), update_doc.clone(), None){
            Ok(result) => {
                if result.acknowledged && result.modified_count > 0 {
                    Ok(statement.clone())
                }else{
                    Err(MMError::new("Unable to add statement to account.", MMErrorKind::DAO))
                }
            },
            Err(e) => {
                warn!("{}", e);
                Err(MMError::new("Failed to insert account statement", MMErrorKind::DAO))
            }
        }
    }// end create
}

/// Create AccountStatementModel from Document
///
/// # Arguments
/// self
/// doc - Document
///
/// # Returns
/// `AccountStatementModel`
fn document_to_model(doc: &Document) -> AccountStatementModel{
    AccountStatementModel{
        statement_date: match doc.get("statement_date"){
            Some(&Bson::UtcDatetime(ref statement_date)) => Some(statement_date.clone()),
            _ => None
        },
        ending_balance: match doc.get("ending_balance"){
            Some(&Bson::FloatingPoint(ref ending_balance)) => Some(ending_balance.clone()),
            _ => None
        }
    }
}
