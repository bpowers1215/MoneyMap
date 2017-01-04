// src/dao/transaction_dao.rs

/// Transaction DAO
/// Handle all database interaction for Transaction collection

// Import
extern crate mongodb;

// Import Modules
// External
use ::chrono::offset::utc::UTC;
// Common Utilities
use ::bson::{Bson, Document};
use ::bson::oid::ObjectId;
use ::mongodb::coll::options::FindOptions;
use ::mongodb::db::ThreadedDatabase;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
// Models
use ::models::transaction_model::{PubTransactionModel, TransactionModel};

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

    /// Create Transaction
    ///
    /// # Arguments
    /// `self`
    /// `pub_transaction` - &PubTransactionModel The transaction
    ///
    /// # Returns
    /// `MMResult<TransactionModel>`
    pub fn create(self, pub_transaction: &PubTransactionModel) -> MMResult<TransactionModel>{
        let coll = self.db.collection(TRANSACTION_COLLECTION);

        let mut transaction = TransactionModel::new(pub_transaction);
        transaction.set_status(Some(String::from("active")));
        transaction.set_datetime(Some(UTC::now()));
        let mut doc = doc!{
            "datetime" => (transaction.get_datetime().unwrap()),
            "payee" => (match transaction.get_payee(){Some(val) => val, None => "".to_string()}),
            "description" => (match transaction.get_description(){Some(val) => val, None => "".to_string()}),
            "amount" => (match transaction.get_amount(){Some(val) => val, None => 0.0}),
            "transaction_type" => (match transaction.get_transaction_type(){Some(val) => val, None => "".to_string()}),
            "status" => (transaction.get_status().unwrap())
        };
        // Set Money Map ID
        if let Some(val) = transaction.get_money_map_id(){
            doc.insert_bson("money_map_id".to_string(), Bson::ObjectId(val));
        }else{
            doc.insert_bson("money_map_id".to_string(), Bson::Null);
        }
        // Set Account ID
        if let Some(val) = transaction.get_account_id(){
            doc.insert_bson("account_id".to_string(), Bson::ObjectId(val));
        }else{
            doc.insert_bson("account_id".to_string(), Bson::Null);
        }

        // Insert document into `transactions` collection
        match coll.insert_one(doc.clone(), None){
            Ok(result) => {
                if result.acknowledged{
                    if let Some(transaction_id) = result.inserted_id{
                        if let Bson::ObjectId(id) = transaction_id{
                            transaction.set_id(id);
                            return Ok(transaction);
                        }
                    }
                }
                Err(MMError::new("Failed to insert transaction", MMErrorKind::DAO))
            },
            Err(e) => {
                warn!("{}", e);
                Err(MMError::new("Failed to insert transaction", MMErrorKind::DAO))
            }
        }
    }// end create

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
