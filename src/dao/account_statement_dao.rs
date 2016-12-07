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
use ::common::utilities as Utilities;
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
    /// `self`
    /// `user_id` - ObjectId User ID
    /// `mm_id` - ObjectId Money Map ID
    /// `acc_id` - ObjectId Money Map ID
    /// `sort` - A Vector of SortParams
    ///
    /// # Returns
    /// `Option<Vec<AccountModel>>`
    pub fn find(self, user_id: ObjectId, mm_id: ObjectId, acc_id: ObjectId, sort: Vec<Utilities::url::SortParam>) -> Option<Vec<AccountStatementModel>>{
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
            Some(&Bson::TimeStamp(ref statement_date)) => Some(statement_date.clone()),
            _ => None
        },
        ending_balance: match doc.get("ending_balance"){
            Some(&Bson::FloatingPoint(ref ending_balance)) => Some(ending_balance.clone()),
            _ => None
        }
    }
}
