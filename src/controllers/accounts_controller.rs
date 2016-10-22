// src/controllers/accounts_controller.rs

/// Accounts Controller

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
use ::models::account_model::{AccountModel};
// DAO
use ::dao::dao_manager::DAOManager;

#[derive(Clone)]
pub struct AccountsController{
    dao_manager: DAOManager,
    config: Config
}

impl AccountsController{

    pub fn new(dao_manager: DAOManager, config: Config) -> AccountsController{
        AccountsController{
            dao_manager: dao_manager,
            config: config
        }
    }

    /// Create Account
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<AccountModel>` - ApiResult including the create account
    pub fn create(&self, req: &mut Request<ServerData>) -> ApiResult<AccountModel, AccountModel>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        ApiResult::Failure{msg:"Not Implemented"}
    }// end create

}
