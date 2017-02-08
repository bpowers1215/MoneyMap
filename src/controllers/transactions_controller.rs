// src/controllers/transactions_controller.rs

/// Transactions Controller

// Import
// External
use ::nickel::{JsonBody, QueryString, Request};
use ::bson::oid::ObjectId;
use ::chrono::{Datelike, Timelike, TimeZone, UTC};
// Utilities
use ::common::api_result::ApiResult;
use ::common::config::Config;
use ::common::data_access::ServerData;
use ::common::session as Session;
// Models
use ::models::transaction_model::{PubTransactionModel};
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
    /// Query params:
    ///     start_date - inclusive (format: YYYYMMDD)
    ///     end_date - exclusive (format: YYYYMMDD)
    ///
    /// # Arguments
    /// `self`
    /// `req` - nickel::Request
    /// `mm_id` - String Money Map ID
    /// `acc_id` - String Account ID
    ///
    /// # Returns
    /// `ApiResult<Vec<PubTransactionModel>>` - ApiResult including a vector of transactions
    pub fn find(&self, req: &mut Request<ServerData>, mm_id: String, acc_id: String) -> ApiResult<Vec<PubTransactionModel>, ()>{
        let start_time = "00:00:00";

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        // Get Query Params
        let query = req.query();

        // Start Date
        let start_date_prop = query.get("start_date");
        let mut start_date = if let Some(date) = start_date_prop{
            let sd = [date, start_time].concat();
            match UTC.datetime_from_str(&sd, "%Y%m%d%T"){
                Ok(datetime) => Some(datetime),
                Err(e) => {
                    error!("{}",e);
                    return ApiResult::Failure{msg:"Unable to parse start date."};
                }
            }
        }else{
            None
        };

        // End Date
        let end_date_prop = query.get("end_date");
        let mut end_date = if let Some(date) = end_date_prop{
            let edt = [date, start_time].concat();
            match UTC.datetime_from_str(&edt, "%Y%m%d%T"){
                Ok(datetime) => Some(datetime),
                Err(e) => {
                    error!("{}",e);
                    return ApiResult::Failure{msg:"Unable to parse end date."};
                }
            }
        }else{
            None
        };

        if let None = start_date{
            if let None = end_date{
                // Start and End dates not supplied
                // Default start/end filter to current month
                let now = UTC::now();

                // Determine first day of this month
                start_date = now.with_day(1).unwrap().with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0);
                // Determine first day of next month
                let (y, m) = if now.month() == 12 { (now.year() + 1, 1) } else { (now.year(), now.month() + 1) };
                end_date = now.with_year(y).unwrap().with_day(1).unwrap().with_month(m).unwrap().with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0);
            }
        }


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

                                        // Verify Account is valid and user has permission
                                        if transaction_dao.is_valid_account(user_obj_id, mm_obj_id.clone(), acc_obj_id.clone()){

                                            // Get list of transactions for account
                                            let transactions = transaction_dao.find(mm_obj_id, acc_obj_id, start_date, end_date);
                                            ApiResult::Success{
                                                result:transactions.into_iter().map(|x| PubTransactionModel::new(x)).collect()
                                            }

                                        }else{
                                            ApiResult::Failure{msg:"Failed to find transactions. Invalid account."}
                                        }
                                    },
                                    Err(e) => {
                                        error!("{}", e);
                                        ApiResult::Failure{msg:"Failed to find transactions. Invalid account ID."}
                                    }
                                }
                            },
                            Err(e) => {
                                error!("{}", e);
                                ApiResult::Failure{msg:"Failed to find transactions. Invalid money map ID."}
                            }
                        }
                    },
                    Err(e) => {
                        error!("{}", e);
                        ApiResult::Failure{msg:"Failed to find transactions. Invalid user ID."}
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
    }// end find_all

    /// Create Transaction
    ///
    /// # Arguments
    /// `self`
    /// `req` - nickel::Request
    /// `mm_id` - String Money Map ID
    /// `acc_id` - String Account ID
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
                                                //TODO Parse error! Floating point numbers are being parsed poorly. Eg. 0.12 -> 0.119999999999999
                                                // This only occurs when the decimal is sent in the request body json as a number, not as a string.
                                                // Temporary workaround: require transaction amount to be passed as a string
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
    /// `self`
    /// `req` - nickel::Request
    /// `mm_id` - String Money Map ID
    /// `acc_id` - String Account ID
    ///
    /// # Returns
    /// `ApiResult<PubTransactionModel>` - ApiResult including the modified transaction
    pub fn modify(&self, req: &mut Request<ServerData>, mm_id: String, acc_id: String) -> ApiResult<PubTransactionModel, PubTransactionModel>{

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

                // START Object ID parsing --------------------------------------------------------
                match ObjectId::with_string(&user_id){
                    Ok(user_obj_id) => {
                        match ObjectId::with_string(&mm_id){
                            Ok(mm_obj_id) => {
                                match ObjectId::with_string(&acc_id){
                                    Ok(acc_obj_id) => {
                                        // END Object ID parsing ----------------------------------

                                        // Verify Account is valid to receive transactions and user has permission
                                        if transaction_dao.is_valid_account(user_obj_id, mm_obj_id.clone(), acc_obj_id.clone()){

                                            // Parse body to PubTransactionModel
                                            match req.json_as::<PubTransactionModel>(){
                                                Ok(transaction) => {

                                                    //Validate
                                                    let validation_result = transaction.validate_existing();
                                                    if validation_result.is_valid(){
                                                        //Save the account
                                                        match transaction_dao.update(&transaction){
                                                            Ok(updated_transaction) => {
                                                                ApiResult::Success{result:PubTransactionModel::new(updated_transaction)}
                                                            },
                                                            Err(e) => {
                                                                ApiResult::Failure{msg:e.get_message()}
                                                            }
                                                        }

                                                    }else{
                                                        ApiResult::Invalid{validation:validation_result, request:transaction}
                                                    }
                                                },
                                                Err(e) => {
                                                    error!("{}",e);
                                                    ApiResult::Failure{msg:"Invalid format. Unable to parse data."}
                                                }
                                            }

                                        }else{
                                            ApiResult::Failure{msg:"Failed to update transaction. Invalid account."}
                                        }

                                        // START Object ID parsing Error Handling -----------------
                                    },
                                    Err(e) => {
                                        error!("{}", e);
                                        ApiResult::Failure{msg:"Failed to find transaction. Invalid account ID."}
                                    }
                                }
                            },
                            Err(e) => {
                                error!("{}", e);
                                ApiResult::Failure{msg:"Failed to find transaction. Invalid money map ID."}
                            }
                        }
                    },
                    Err(e) => {
                        error!("{}", e);
                        ApiResult::Failure{msg:"Failed to find transaction. Invalid user ID."}
                    }
                }
                // END Object ID parsing Error Handling -------------------------------------------

                // START Retrieve DAO Error Handling ----------------------------------------------
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
        // END Retrieve DAO Error Handling --------------------------------------------------------
    }// end modify

    /// Delete a Transaction
    ///
    /// # Arguments
    /// `self`
    /// `req` - &nickel::Request
    ///
    /// # Returns
    /// `ApiResult<String>` - ApiResult
    pub fn delete(&self, req: &Request<ServerData>, mm_id: String, acc_id: String, tran_id: String) -> ApiResult<String, ()>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        match self.dao_manager.get_transaction_dao(){
            Ok(dao) => {

                // START Object ID parsing --------------------------------------------------------
                match ObjectId::with_string(&user_id){
                    Ok(user_obj_id) => {
                        match ObjectId::with_string(&mm_id){
                            Ok(mm_obj_id) => {
                                match ObjectId::with_string(&acc_id){
                                    Ok(acc_obj_id) => {
                                        match ObjectId::with_string(&tran_id){
                                            Ok(tran_obj_id) => {
                                                // END Object ID parsing --------------------------

                                                // Verify Account/money map is valid and user has permission
                                                if dao.is_valid_account(user_obj_id, mm_obj_id.clone(), acc_obj_id.clone()){

                                                    if dao.delete(mm_obj_id, acc_obj_id, tran_obj_id){
                                                        ApiResult::Success{result:"Successfully deleted transaction".to_string()}
                                                    }else{
                                                        ApiResult::Failure{msg:"Failed to delete transaction."}
                                                    }

                                                }else{
                                                    ApiResult::Failure{msg:"Failed to delete transaction. Invalid account."}
                                                }

                                                // START Object ID parsing Error Handling ---------
                                            },
                                            Err(e) => {
                                                error!("{}", e);
                                                ApiResult::Failure{msg:"Failed to delete transaction. Invalid transaction ID."}
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        error!("{}", e);
                                        ApiResult::Failure{msg:"Failed to delete transaction. Invalid account ID."}
                                    }
                                }
                            },
                            Err(e) => {
                                error!("{}", e);
                                ApiResult::Failure{msg:"Failed to delete transaction. Invalid money map ID."}
                            }
                        }
                    },
                    Err(e) => {
                        error!("{}", e);
                        ApiResult::Failure{msg:"Failed to delete transaction. Invalid user ID."}
                    }
                }
                // END Object ID parsing Error Handling -------------------------------------------
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
    }// end delete
}
