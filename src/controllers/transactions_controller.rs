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

        // START Retrieve DAO ---------------------------------------------------------------------
        match self.dao_manager.get_transaction_dao(){
            Ok(transaction_dao) => {
                // END Retrieve DAO ---------------------------------------------------------------

                match ObjectId::with_string(&user_id){
                    Ok(user_obj_id) => {
                        match ObjectId::with_string(&mm_id){
                            Ok(mm_obj_id) => {
                                match ObjectId::with_string(&acc_id){
                                    Ok(acc_obj_id) => {

                                        // Verify Account is valid to receive transactions and user has permission
                                        if transaction_dao.is_valid_account(user_obj_id, mm_obj_id.clone(), acc_obj_id.clone()){

                                            // Parse body to PubTransactionModel
                                            match req.json_as::<PubTransactionModel>(){
                                                Ok(mut pub_transaction) => {

                                                    pub_transaction.set_money_map_id(Some(mm_obj_id));
                                                    pub_transaction.set_account_id(Some(acc_obj_id));

                                                    // Validate
                                                    let validation_result = pub_transaction.validate();
                                                    if validation_result.is_valid(){
                                                        // Save Transaction
                                                        match transaction_dao.create(&pub_transaction){
                                                            Ok(new_transaction) => {
                                                                ApiResult::Success{result:PubTransactionModel::new(new_transaction)}
                                                            },
                                                            Err(e) => {
                                                                error!("{}",e);
                                                                ApiResult::Failure{msg:"Unable to create account"}
                                                            }
                                                        }
                                                    }else{
                                                        ApiResult::Invalid{validation:validation_result, request:pub_transaction}
                                                    }
                                                },
                                                Err(e) => {
                                                    error!("{}",e);
                                                    ApiResult::Failure{msg:"Invalid format. Unable to parse body."}
                                                }
                                            }

                                        }else{
                                            ApiResult::Failure{msg:"Failed to create transaction. Invalid account."}
                                        }

                                    },
                                    Err(e) => {
                                        error!("{}", e);
                                        ApiResult::Failure{msg:"Failed to create transaction. Invalid account ID."}
                                    }
                                }
                            },
                            Err(e) => {
                                error!("{}", e);
                                ApiResult::Failure{msg:"Failed to create transaction. Invalid money map ID."}
                            }
                        }
                    },
                    Err(e) => {
                        error!("{}", e);
                        ApiResult::Failure{msg:"Failed to create transaction. Invalid user ID."}
                    }
                }

                // START Retrieve DAO Error Handling ----------------------------------------------
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
        // END Retrieve DAO Error Handling --------------------------------------------------------
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
