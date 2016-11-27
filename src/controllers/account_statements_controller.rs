// src/controllers/account_statements_controller.rs

/// Account Statements Controller

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
use ::models::account_statement_model::{AccountStatementModel};
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
    ///
    /// # Returns
    /// `ApiResult<Vec<AccountStatementModel>>` - ApiResult including a vector of account statements
    pub fn find(&self, req: &mut Request<ServerData>, mm_id: String) -> ApiResult<Vec<AccountStatementModel>, ()>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };
        ApiResult::Failure{msg:"Needs Implemented."}
    }// end find_all

}
