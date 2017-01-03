// src/controllers/transactions_controller.rs

/// Transactions Controller

// Import
// External
use ::nickel::{JsonBody, Request};
use ::bson::{Bson};
use ::bson::oid::ObjectId;
// Utilities
use ::common::api_result::ApiResult;
use ::common::config::Config;
use ::common::data_access::ServerData;
use ::common::session as Session;
// Models
use ::models::transaction_model::{TransactionModel, PubTransactionModel};
// DAO
use ::dao::dao_manager::DAOManager;

#[derive(Clone)]
pub struct TransactionsController{
    dao_manager: DAOManager,
    config: Config
}

impl TransactionsController{

    pub fn new(dao_manager: DAOManager, config: Config) -> TransactionsController{
        TransactionsController{
            dao_manager: dao_manager,
            config: config
        }
    }

    /// Find All Transactions belonging to a money map account
    /// Utilize query params to filter results. Defaults to transactions for current month.
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    /// mm_id - String Money Map ID
    /// acc_id - String Account ID
    ///
    /// # Returns
    /// `ApiResult<Vec<PubTransactionModel>>` - ApiResult including a vector of transactions
    pub fn find(&self, req: &mut Request<ServerData>, mm_id: String, acc_id: String) -> ApiResult<Vec<PubTransactionModel>, ()>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        ApiResult::Failure{msg:"Method not implemented"}
    }// end find_all

    /// Create Transaction
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    /// mm_id - String Money Map ID
    /// acc_id - String Account ID
    ///
    /// # Returns
    /// `ApiResult<PubTransactionModel>` - ApiResult including the created transaction
    pub fn create(&self, req: &mut Request<ServerData>, mm_id: String, acc_id: String) -> ApiResult<PubTransactionModel, PubTransactionModel>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        ApiResult::Failure{msg:"Method not implemented"}
    }// end create

    /// Modify TransactionModel
    /// Transaction Amount and Type (credit/debit) can only be modified during transaction month
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<PubTransactionModel>` - ApiResult including the modified transaction
    pub fn modify(&self, req: &mut Request<ServerData>) -> ApiResult<PubTransactionModel, PubTransactionModel>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        ApiResult::Failure{msg:"Method not implemented"}
    }// end modify

    /// Delete a Transaction
    ///
    /// # Arguments
    /// &self
    /// req - &nickel::Request
    ///
    /// # Returns
    /// `ApiResult<String>` - ApiResult
    pub fn delete(&self, req: &Request<ServerData>) -> ApiResult<String, ()>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        ApiResult::Failure{msg:"Method not implemented"}
    }// end delete
}
