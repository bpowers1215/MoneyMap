// src/dao/transaction_dao.rs

/// Transaction DAO
/// Handle all database interaction for Transaction collection

// Import
extern crate mongodb;

// Import Modules
// External
use ::chrono::{DateTime};
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

    /// Find Transactions for Account
    /// Allow filtering transactions by date
    /// Sorts transactions by date in descending order
    ///
    /// # Arguments
    /// `self`
    /// `user_id` - ObjectId User ID
    /// `mm_id` - ObjectId Money Map ID
    /// `acc_id` - ObjectId Money Map ID
    /// `start_date` - Start DateTime - inclusive
    /// `end_date` - End DateTime - exclusive
    ///
    /// # Returns
    /// `Vec<TransactionModel>`
    pub fn find(self, mm_id: ObjectId, acc_id: ObjectId, start_date: Option<DateTime<UTC>>, end_date: Option<DateTime<UTC>>) -> Vec<TransactionModel>{
        let coll = self.db.collection(TRANSACTION_COLLECTION);
        let mut transactions = Vec::new();

        let mut find_options = FindOptions::new();
        find_options.sort = Some(doc!{
            "datetime" => (-1)
        });
        let mut filter = doc!{
            "money_map_id" => mm_id,
            "account_id" => acc_id
        };

        let mut date_query = None;
        // Start Date
        if let Some(sd) = start_date{
            let mut date_condition = doc!{};
            if let Some(dq) = date_query{
                date_condition = dq;
            }
            date_condition.insert_bson("$gte".to_string(), Bson::UtcDatetime(sd));
            date_query = Some(date_condition);
        }
        // End Date
        if let Some(ed) = end_date{
            let mut date_condition = doc!{};
            if let Some(dq) = date_query{
                date_condition = dq;
            }
            date_condition.insert_bson("$lt".to_string(), Bson::UtcDatetime(ed));
            date_query = Some(date_condition);
        }

        // Add date query to filter
        if let Some(dq) = date_query{
            filter.insert_bson("datetime".to_string(), Bson::Document(dq));
        }

        match coll.find(Some(filter), Some(find_options)){
            Ok(cursor) => {
                for result in cursor {
                    if let Ok(item) = result {
                        let transaction = document_to_model(&item);
                        transactions.push(transaction);
                    }
                }
            },
            Err(e) => {
                error!("Find All money_maps failed: {}", e)
            }
        }
        transactions
    }// end find

    /// Find One Transaction
    ///
    /// # Arguments
    /// self
    /// filter - Option<Document> The find filter
    ///
    /// # Returns
    /// `Option<TransactionModel>`
    pub fn find_one(&self, filter: Option<Document>) -> Option<TransactionModel>{
        let coll = self.db.collection(TRANSACTION_COLLECTION);

        match coll.find_one(filter, None){
            Ok(result) => {
                match result{
                    Some(document) => {
                        Some(document_to_model(&document))
                    },
                    None => {
                        //Could not find money map for user
                        None
                    }
                }
            },
            Err(e) => {
                error!("Find All accounts failed: {}", e);
                None
            }
        }
    }// end find_one

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

    /// Update an existing transaction
    ///
    /// # Arguments
    /// `self`
    /// `pub_transaction` - PubTransactionModel
    ///
    /// # Returns
    /// `MMResult<TransactionModel>` The updated transaction if successful, None otherwise
    pub fn update(&self, pub_transaction: &PubTransactionModel) -> MMResult<TransactionModel>{
        let coll = self.db.collection(TRANSACTION_COLLECTION);

        let filter = doc! {
            "_id" => (pub_transaction.get_id().unwrap())
        };

        // Build `$set` document to update document
        let mut set_doc = doc!{};
        let mut update = false;
        if let Some(payee) = pub_transaction.get_payee(){
            update = true;
            set_doc.insert_bson("payee".to_string(), Bson::String(payee));
        }
        if let Some(description) = pub_transaction.get_description(){
            update = true;
            set_doc.insert_bson("description".to_string(), Bson::String(description));
        }
        if let Some(amount) = pub_transaction.get_amount(){
            update = true;
            set_doc.insert_bson("amount".to_string(), Bson::FloatingPoint(amount));
        }
        if let Some(transaction_type) = pub_transaction.get_transaction_type(){
            update = true;
            set_doc.insert_bson("transaction_type".to_string(), Bson::String(transaction_type));
        }
        if let Some(category_id) = pub_transaction.get_category_id(){
            update = true;
            set_doc.insert_bson("category_id".to_string(), Bson::ObjectId(category_id));
        }
        if let Some(status) = pub_transaction.get_status(){
            update = true;
            set_doc.insert_bson("status".to_string(), Bson::String(status));
        }
        let update_doc = if update {
            doc! {"$set" => set_doc}
        }else{
            // No updates to transaction, return existing transaction if found
            return match self.find_one(Some(filter)){
                Some(result) => Ok(result),
                None => Err(MMError::new("Unable to find transaction", MMErrorKind::DAO))
            };
        };

        // Update the transaction
        match coll.update_one(filter.clone(), update_doc.clone(), None){
            Ok(result) => {
                match result.write_exception{
                    None => {
                        if result.matched_count > 0{
                            // Transaction found and updated
                            Ok(self.find_one(Some(filter)).unwrap())
                        }else{
                            Err(MMError::new("Unable to find transaction", MMErrorKind::DAO))
                        }
                    },
                    Some(_) => {
                        Err(MMError::new("Unable to save transaction", MMErrorKind::DAO))
                    }
                }

            },
            Err(e) => {
                error!("{}", e);
                Err(MMError::new("Failed to update transaction.", MMErrorKind::DAO))
            }
        }
    }// end update

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
                        //Could not find transaction
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

/// Create TransactionModel from Document
///
/// # Arguments
/// self
/// doc - Document
///
/// # Returns
/// `TransactionModel`
fn document_to_model(doc: &Document) -> TransactionModel{
    TransactionModel{
        id: match doc.get("_id"){
            Some(obj_id) => match obj_id{ &Bson::ObjectId(ref id) => Some(id.clone()), _ => None},
            _ => None
        },
        money_map_id: match doc.get("money_map_id"){
            Some(obj_id) => match obj_id{ &Bson::ObjectId(ref id) => Some(id.clone()), _ => None},
            _ => None
        },
        account_id: match doc.get("account_id"){
            Some(obj_id) => match obj_id{ &Bson::ObjectId(ref id) => Some(id.clone()), _ => None},
            _ => None
        },
        datetime: match doc.get("datetime"){
            Some(&Bson::UtcDatetime(ref statement_date)) => Some(statement_date.clone()),
            _ => None
        },
        payee: match doc.get("payee"){
            Some(&Bson::String(ref name)) => Some(name.clone()),
            _ => None
        },
        description: match doc.get("description"){
            Some(&Bson::String(ref name)) => Some(name.clone()),
            _ => None
        },
        category_id: match doc.get("category_id"){
            Some(obj_id) => match obj_id{ &Bson::ObjectId(ref id) => Some(id.clone()), _ => None},
            _ => None
        },
        amount: match doc.get("amount"){
            Some(&Bson::FloatingPoint(ref ending_balance)) => Some(ending_balance.clone()),
            _ => None
        },
        transaction_type: match doc.get("transaction_type"){
            Some(&Bson::String(ref name)) => Some(name.clone()),
            _ => None
        },
        status: match doc.get("status"){
            Some(&Bson::String(ref name)) => Some(name.clone()),
            _ => None
        },
    }
}
