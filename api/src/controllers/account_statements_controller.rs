// src/controllers/account_statements_controller.rs

/// Account Statements Controller

// Import
// External
use ::nickel::{QueryString, Request};
use ::bson::oid::ObjectId;
use ::chrono::{TimeZone};
use ::chrono::offset::utc::UTC;
// Utilities
use ::common::api_result::ApiResult;
use ::common::config::Config;
use ::common::data_access::ServerData;
use ::common::session as Session;
use ::common::utilities as Utilities;
// Models
use ::models::account_model::{AccountModel};
use ::models::money_map_model::{MoneyMapModel};
use ::models::account_statement_model::{OutAccountStatementModel};
use ::models::transaction_model::{TransactionModel};
// DAO
use ::dao::dao_manager::DAOManager;

#[derive(Clone)]
pub struct AccountStatementsController{
    dao_manager: DAOManager,
    config: Config
}

impl AccountStatementsController{

    pub fn new(dao_manager: DAOManager, config: Config) -> AccountStatementsController{
        AccountStatementsController{
            dao_manager: dao_manager,
            config: config
        }
    }

    /// Retrieve all account statements
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    /// mm_id - String Money Map ID
    /// acc_id - String Account ID
    ///
    /// # Returns
    /// `ApiResult<Vec<OutAccountStatementModel>>` - ApiResult including a vector of account statements
    pub fn find(&self, req: &mut Request<ServerData>, mm_id: String, acc_id: String) -> ApiResult<Vec<OutAccountStatementModel>, ()>{
        let start_time = "00:00:00";
        let end_time = "23:59:59";

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        // Get Query Params
        let query = req.query();
        // Sort: Default statement_date descending
        let sort_prop = query.get("sort").unwrap_or("-statement_date");
        let sort = Utilities::url::get_sort_params(sort_prop);

        // Start Date
        let start_date_prop = query.get("start_date");
        let start_date = if let Some(date) = start_date_prop{
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
        let end_date = if let Some(date) = end_date_prop{
            let edt = [date, end_time].concat();
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

        // START Retrieve DAO ---------------------------------------------------------------------
        match self.dao_manager.get_account_statement_dao(){
            Ok(account_statement_dao) => {
                // END Retrieve DAO ---------------------------------------------------------------

                match ObjectId::with_string(&user_id){
                    Ok(user_obj_id) => {
                        match ObjectId::with_string(&mm_id){
                            Ok(mm_obj_id) => {
                                match ObjectId::with_string(&acc_id){
                                    Ok(acc_obj_id) => {

                                        // Get list of accounts for money map
                                        match account_statement_dao.find(user_obj_id, mm_obj_id, acc_obj_id, sort, start_date, end_date){
                                            Some(accounts) => {
                                                ApiResult::Success{
                                                    result:accounts.into_iter().map(|x| OutAccountStatementModel::new(x)).collect()
                                                }
                                            },
                                            None => {
                                                ApiResult::Failure{msg:"Unable to find money map."}
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        error!("{}", e);
                                        ApiResult::Failure{msg:"Failed to find money map. Invalid account ID."}
                                    }
                                }
                            },
                            Err(e) => {
                                error!("{}", e);
                                ApiResult::Failure{msg:"Failed to find money map. Invalid money map ID."}
                            }
                        }
                    },
                    Err(e) => {
                        error!("{}", e);
                        ApiResult::Failure{msg:"Failed to find money map. Invalid user ID."}
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


    /// Generate Account Statements for all active accounts
    ///
    /// # Arguments
    /// &self
    pub fn generate_statements(&self){
        debug!("CREATE STATEMENTS");

        //For each active money map
        {
            //For each active account
            {
                //Get the latest statement
                //IF last months statement has not yet been created:
                {
                    //Get last months transactions
                    //Calculate last months ending account balance
                        // Last Month's ending bal = prev month's ending bal + (total last months transactions)

                    //Create account statement for last month from calculated ending balance
                }
            }
        }
    }
    
    /// Calculate new ending balance from previous ending balance and list of transactions
    ///
    /// # Arguments
    /// balance - f64
    /// transactions - Vec<TransactionModel>
    ///
    /// # Returns
    /// `f64` - ending account balance
    fn calculate_ending_balance(mut balance: f64, transactions: Vec<TransactionModel>) -> f64 {
        for transaction in &transactions {
            if let Some(t_type) = transaction.get_transaction_type(){
                if let Some(t_amount) = transaction.get_amount(){
                    println!("Type: {}; Amount: {}", t_type, t_amount);
                    match t_type.as_ref() {
                        "credit" => {
                            balance += t_amount
                        },
                        "debit" => {
                            balance -= t_amount
                        },
                        _ => {}
                    }
                }
            }
        }
        balance
    }

    /// Testing: Create Account statement
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    /// mm_id - String Money Map ID
    /// acc_id - String Account ID
    ///
    /// # Returns
    /// `ApiResult<Vec<OutAccountStatementModel>>` - ApiResult including a vector of account statements
    pub fn test_create_account_statement(&self, req: &mut Request<ServerData>, mm_id: String, acc_id: String) -> ApiResult<Vec<OutAccountStatementModel>, ()>{
        let start_time = "00:00:00";
        let end_time = "23:59:59";

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

         return ApiResult::Failure{msg:"Not Implemented"};

    }// end test_create_account_statement

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_ending_balance() {

        let mut transactions = Vec::new();

        // Test 1
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(1.00),
            transaction_type: Some("debit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(2.00),
            transaction_type: Some("credit".to_string()),
            status: None
        });
        assert_eq!(6.00, AccountStatementsController::calculate_ending_balance(5.00, transactions.clone()));

        // Test 2
        transactions.clear();
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(1.0),
            transaction_type: Some("credit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(2.50),
            transaction_type: Some("credit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(3.64),
            transaction_type: Some("credit".to_string()),
            status: None
        });
        assert_eq!(9.29, AccountStatementsController::calculate_ending_balance(2.15, transactions.clone()));

        // Test 3
        transactions.clear();
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(150.75),
            transaction_type: Some("debit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(25.31),
            transaction_type: Some("credit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(5.63),
            transaction_type: Some("debit".to_string()),
            status: None
        });
        transactions.push(TransactionModel{
            id: None,
            money_map_id: None,
            account_id: None,
            datetime: None,
            payee: None,
            description: None,
            category_id: None,
            amount: Some(150.00),
            transaction_type: Some("debit".to_string()),
            status: None
        });
        assert_eq!(720.17, AccountStatementsController::calculate_ending_balance(1001.24, transactions.clone()));
    }
}