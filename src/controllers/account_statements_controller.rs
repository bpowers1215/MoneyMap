// src/controllers/account_statements_controller.rs

/// Account Statements Controller

// Import
// External
use ::nickel::{JsonBody, QueryString, Request};
use ::bson::{Bson};
use ::bson::oid::ObjectId;
use ::chrono::{DateTime, Duration, Local, TimeZone};
use ::chrono::offset::utc::UTC;
// Utilities
use ::common::api_result::ApiResult;
use ::common::config::Config;
use ::common::data_access::ServerData;
use ::common::session as Session;
use ::common::utilities as Utilities;
// Models
use ::models::account_statement_model::{AccountStatementModel, OutAccountStatementModel};
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
        let START_TIME = "00:00:00";
        let END_TIME = "23:59:59";

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
            let sd = [date, START_TIME].concat();
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
            let edt = [date, END_TIME].concat();
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

}
