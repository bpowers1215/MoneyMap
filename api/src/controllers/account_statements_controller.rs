// src/controllers/account_statements_controller.rs

/// Account Statements Controller

// Import
// External
use ::nickel::{QueryString, Request};
use ::bson::oid::ObjectId;
use ::std::error::Error;
use ::chrono::{Datelike, TimeZone};
use ::chrono::Utc as UTC;
// Utilities
use ::common::api_result::ApiResult;
use ::common::config::Config;
use ::common::data_access::ServerData;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::common::session as Session;
use ::common::utilities as Utilities;
// Models
use ::models::account_model::{AccountModel};
use ::models::money_map_model::{MoneyMapModel, PubMoneyMapModel};
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

                                match self.generate_account_statement(user_obj_id, mm_obj_id, acc_obj_id, 2017, 6){
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

    /// Testing: Get all money maps
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<Vec<MoneyMapModel>>` - ApiResult including a vector of money maps
    pub fn test_get_all_money_maps(&self, req: &mut Request<ServerData>) -> ApiResult<Vec<PubMoneyMapModel>, ()>{
        // START Retrieve DAO ---------------------------------------------------------------------
         match self.dao_manager.get_money_map_dao(){
            Ok(money_map_dao) => {

                self.generate_statements();

                // Get all Active Money Maps/Accounts
                let money_maps = money_map_dao.find(None);
                let mut pub_money_maps = Vec::new();

                // Convert to public money maps
                for i in 0..money_maps.len(){
                    pub_money_maps.push(PubMoneyMapModel::new(money_maps[i].clone()))
                }

                ApiResult::Success{
                    result: pub_money_maps
                }

                // For each active money map
                /*for money_map in money_maps {
                    // For each active account
                    {
                        // Generate Account statement
                    }
                }*/

            // START Retrieve DAO Error Handling --------------------------------------------------
            },
            Err(e) => {
                ApiResult::Failure{msg:"Failed to get money maps"}
            }
        }
        // END Retrieve DAO Error Handling --------------------------------------------------------
    }// end test_get_all_money_maps


    /// Generate Account Statements for all active accounts
    ///
    /// # Arguments
    /// &self
    pub fn generate_statements(&self){

         // START Retrieve DAO ---------------------------------------------------------------------
         match self.dao_manager.get_money_map_dao(){
            Ok(money_map_dao) => {

                // Get all Active Money Maps/Accounts
                let money_maps = money_map_dao.find(None);

                // For each active money map
                for money_map in money_maps {
                    
                    // Get money map ID
                    if let Some(money_map_id) = money_map.get_id(){
                        // Get a money map user
                        if let Some(users) = money_map.get_users(){
                            if users.len() > 0 {
                                if let Some(user) = users[0].get_user(){
                                    if let Some(user_id) = user.get_id(){

                                        // For each active account
                                        if let Some(accounts) = money_map.get_accounts(){
                                            for account in accounts {
                                                
                                                // Get Account ID
                                                if let Some(account_id) = account.get_id(){
                                                    let now = UTC::now();
                                                    // Generate Account statement
                                                    self.generate_account_statement( ObjectId::with_string(&user_id.clone()).unwrap(),  money_map_id.clone(),  ObjectId::with_string(&account_id.clone()).unwrap(), now.year(), now.month());
                                                }

                                            }
                                        }
                                    }
                                }
                            }
                        }

                    }
                }

            // START Retrieve DAO Error Handling --------------------------------------------------
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
            }
        }
        // END Retrieve DAO Error Handling --------------------------------------------------------
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
    fn generate_account_statement(&self, user_obj_id: ObjectId, mm_obj_id: ObjectId, acc_obj_id: ObjectId, year: i32, month: u32) -> MMResult<AccountStatementModel>{

        // START Retrieve DAO ---------------------------------------------------------------------
        match self.dao_manager.get_account_statement_dao(){
            Ok(account_statement_dao) => {
                match self.dao_manager.get_transaction_dao(){
                    Ok(transaction_dao) => {
                        // END Retrieve DAO ---------------------------------------------------------------

                        // Calculate date range for current statement query
                        let (next_year, next_month) = if month == 12 { (year + 1, 1) } else { (year, month + 1)};
                        let sort = vec![Utilities::url::SortParam{
                            field: "statement_date".to_string(),
                            direction: 1
                        }];
                        let start_date_string = &[&year.to_string(), "-", &format!("{:02}", month), "-01 00:00:00"].concat();
                        let start_date = match UTC.datetime_from_str(start_date_string, "%F %T"){
                            Ok(result) => result,
                            Err(e) => {
                                error!("{}", e);
                                return Err(MMError::new("Could not parse start date", MMErrorKind::Controller));
                            }
                        };
                        let end_date_string = &[&next_year.to_string(), "-", &format!("{:02}", next_month), "-01 00:00:00"].concat();
                        let end_date = match UTC.datetime_from_str(end_date_string, "%F %T"){
                            Ok(result) => result,
                            Err(e) => {
                                return Err(MMError::new("Could not parse end date", MMErrorKind::Controller));
                            }
                        };

                        // Check if an account statement already exists for this month
                        match account_statement_dao.find(user_obj_id.clone(), mm_obj_id.clone(), acc_obj_id.clone(), sort.clone(), Some(start_date), Some(end_date)){
                            Some(current_month_statements) => {
                                
                                if current_month_statements.len() > 0 {
                                    // Account statement already exists for this month
                                    Err(MMError::new("Account statement already exists", MMErrorKind::Controller))
                                } else {
                                    // No account statements found for this month, continue with statement generation

                                    // Calculate Date range for previous statement query
                                    let (prev_year, prev_month) = if month == 1 { (year - 1, 12) } else { (year, month - 1)};
                                    let start_date_string = &[&prev_year.to_string(), "-", &format!("{:02}", prev_month), "-01 00:00:00"].concat();
                                    let start_date = match UTC.datetime_from_str(start_date_string, "%F %T"){
                                        Ok(result) => result,
                                        Err(e) => {
                                            error!("{}", e);
                                            return Err(MMError::new("Could not parse start date", MMErrorKind::Controller));
                                        }
                                    };
                                    let end_date_string = &[&year.to_string(), "-", &format!("{:02}", month), "-01 00:00:00"].concat();
                                    let end_date = match UTC.datetime_from_str(end_date_string, "%F %T"){
                                        Ok(result) => result,
                                        Err(e) => {
                                            return Err(MMError::new("Could not parse end date", MMErrorKind::Controller));
                                        }
                                    };
                                    // Get previous months account statement
                                    match account_statement_dao.find(user_obj_id.clone(), mm_obj_id.clone(), acc_obj_id.clone(), sort, Some(start_date), Some(end_date)){
                                        Some(prev_month_statements) => {
                                            if prev_month_statements.len() == 0 {
                                                Err(MMError::new("No account statement found for previous month", MMErrorKind::Controller))
                                            } else if prev_month_statements.len() > 1 {
                                                Err(MMError::new("Multiple account statements for previous month", MMErrorKind::Controller))
                                            } else {

                                                //Get last months transactions
                                                let transactions = transaction_dao.find(mm_obj_id.clone(), acc_obj_id.clone(), Some(start_date), Some(end_date));
                                                debug!("{:?}", transactions);

                                                let previous_balance = match prev_month_statements[0].get_ending_balance(){
                                                    Some(bal) => bal,
                                                    None => 0f64
                                                };
                                                let new_account_statement = AccountStatementModel::generate_account_statement(previous_balance, transactions);
                                                //Generate account statement
                                                //Save account statement
                                                match account_statement_dao.create(&new_account_statement, mm_obj_id.clone(), acc_obj_id.clone()) {
                                                    Ok(result) => Ok(new_account_statement),
                                                    Err(e) => {
                                                        error!("{}",e.get_message().to_string());
                                                        Err(MMError::new("Error saving account statement", MMErrorKind::Controller))
                                                    }
                                                }
                                            }
                                        },
                                        None => {
                                            Err(MMError::new("Error retrieving previous month's account statement", MMErrorKind::Controller))
                                        }
                                    }
                                }
                            },
                            None => {
                                Err(MMError::new("Error retrieving current month's account statements", MMErrorKind::Controller))
                            }
                        }

                        // START Retrieve DAO Error Handling ----------------------------------------------
                    },
                    Err(e) => {
                        error!("{}",e.get_message().to_string());
                        Err(MMError::new("Unable to interact with database", MMErrorKind::Controller))
                    }
                }
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
                Err(MMError::new("Unable to interact with database", MMErrorKind::Controller))
            }
        }
        // END Retrieve DAO Error Handling --------------------------------------------------------
    }//end generate_account_statement

}