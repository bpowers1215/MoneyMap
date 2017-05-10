// src/controllers/account_statements_controller.rs

/// Account Statements Controller

// Import
// External
use ::nickel::{QueryString, Request};
use ::bson::oid::ObjectId;
use ::chrono::{TimeZone};
use ::chrono::offset::utc::UTC;
use ::chrono::{Duration};
// Utilities
use ::common::api_result::ApiResult;
use ::common::config::Config;
use ::common::data_access::ServerData;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::common::session as Session;
use ::common::utilities as Utilities;
// Models
use ::models::account_model::{AccountModel};
use ::models::money_map_model::{MoneyMapModel};
use ::models::account_statement_model::{AccountStatementModel, OutAccountStatementModel};
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

        match ObjectId::with_string(&user_id){
            Ok(user_obj_id) => {
                match ObjectId::with_string(&mm_id){
                    Ok(mm_obj_id) => {
                        match ObjectId::with_string(&acc_id){
                            Ok(acc_obj_id) => {

                                match self.generate_account_statement(user_obj_id, mm_obj_id, acc_obj_id, 2017, 3){
                                    Ok(result) => {
                                        debug!("RESULT: {:?}", result);
                                        ApiResult::Success{
                                            result: vec![OutAccountStatementModel::new(result)]
                                        }
                                    },
                                    Err(e) => {
                                        error!("{}", e);
                                        ApiResult::Failure{msg:"Failed to generate account statement."}
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

    }// end test_create_account_statement


    /// Generate Account Statements for all active accounts
    ///
    /// # Arguments
    /// &self
    pub fn generate_statements(&self){
        debug!("generate_statements");

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


    /// Generate Account Statement for an account
    /// (An account statement for any given month will contain the previous months ending balance)
    ///
    /// # Arguments
    /// &self
    /// user_obj_id - ObjectId
    /// mm_obj_id - ObjectId
    /// acc_obj_id - ObjectId
    /// year - f64
    /// month - f32
    fn generate_account_statement(&self, user_obj_id: ObjectId, mm_obj_id: ObjectId, acc_obj_id: ObjectId, year: i32, month: i32) -> MMResult<AccountStatementModel>{
        debug!("generate_account_statement");

        // START Retrieve DAO ---------------------------------------------------------------------
        match self.dao_manager.get_account_statement_dao(){
            Ok(account_statement_dao) => {
                // END Retrieve DAO ---------------------------------------------------------------

                // Get previous months account statement
                // Determine first day of next month
                let (prev_year, prev_month) = if month == 1 { (year - 1, 12) } else { (year, month - 1)};
                let sort = vec![Utilities::url::SortParam{
                    field: "statement_date".to_string(),
                    direction: 1
                }];
                let start_date_string = &[&prev_year.to_string(), "-", &format!("{:02}", prev_month), "-01 00:00:00"].concat();
                let start_date = match UTC.datetime_from_str(start_date_string, "%F %T"){
                    Ok(result) => result,
                    Err(e) => {
                        error!("{}", e);
                        return Err(MMError::new("Could not parse start date", MMErrorKind::Controller));
                    }
                };
                let end_date_string = &[&prev_year.to_string(), "-", &format!("{:02}", month), "-01 00:00:00"].concat();
                let end_date = match UTC.datetime_from_str(end_date_string, "%F %T"){
                    Ok(result) => result - Duration::nanoseconds(1),
                    Err(e) => {
                        return Err(MMError::new("Could not parse end date", MMErrorKind::Controller));
                    }
                };

                match account_statement_dao.find(user_obj_id, mm_obj_id, acc_obj_id, sort, Some(start_date), Some(end_date)){
                    Some(statements) => {
                        if statements.len() == 0 {
                            Err(MMError::new("No account statement found for previous month", MMErrorKind::Controller))
                        } else if statements.len() > 1 {
                            Err(MMError::new("Multiple account statements for previous month", MMErrorKind::Controller))
                        } else {
                            Ok(statements[0].clone())// previous month's account statement

                            //IF last months account statement does not exist, throw error
                            //Get last months transactions
                            //Generate account statement
                            //Save account statement
                        }
                    },
                    None => {
                        Err(MMError::new("Error retrieving account statements", MMErrorKind::Controller))
                    }
                }

                // START Retrieve DAO Error Handling ----------------------------------------------
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
                Err(MMError::new("Unable to interact with database", MMErrorKind::Controller))
            }
        }
        // END Retrieve DAO Error Handling --------------------------------------------------------
    }//end generate_account_statement

}