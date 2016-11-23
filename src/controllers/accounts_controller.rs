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
use ::models::account_model::{AccountModel, OutAccountModel};
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
    /// mm_id - String Money Map ID
    ///
    /// # Returns
    /// `ApiResult<OutAccountModel, AccountModel>` - ApiResult including the created account
    pub fn create(&self, req: &mut Request<ServerData>, mm_id: String) -> ApiResult<OutAccountModel, AccountModel>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        // START Retrieve DAO ---------------------------------------------------------------------
        match self.dao_manager.get_money_map_dao(){
            Ok(mm_dao) => {
                match self.dao_manager.get_account_dao(){
                    Ok(account_dao) => {
                        // END Retrieve DAO -------------------------------------------------------

                        match ObjectId::with_string(&mm_id){
                            Ok(mm_obj_id) => {
                                match ObjectId::with_string(&user_id){
                                    Ok(user_obj_id) => {

                                        let filter = doc!{
                                            "_id" => (mm_obj_id.clone()),
                                            "users.user_id" => user_obj_id,
                                            "deleted" => {
                                                "$ne" => true
                                            }
                                        };
                                        match mm_dao.find_one(Some(filter), None){
                                            Some(_) => {

                                                // Parse body to AccountModel
                                                match req.json_as::<AccountModel>(){
                                                    Ok(account) => {

                                                        // Validate
                                                        let validation_result = account.validate_new();
                                                        if validation_result.is_valid(){
                                                            // Save Account
                                                            match account_dao.create(mm_obj_id, &account){
                                                                Ok(new_account) => {

                                                                    ApiResult::Success{result:OutAccountModel::new(new_account)}
                                                                },
                                                                Err(e) => {
                                                                    error!("{}",e);
                                                                    ApiResult::Failure{msg:"Unable to create account"}
                                                                }
                                                            }
                                                        }else{
                                                            ApiResult::Invalid{validation:validation_result, request:account}
                                                        }

                                                    },
                                                    Err(e) => {
                                                        error!("{}",e);
                                                        ApiResult::Failure{msg:"Invalid format. Unable to parse data."}
                                                    }
                                                }
                                            },
                                            None => {
                                                ApiResult::Failure{msg:"Unable to find money map."}
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        error!("{}", e);
                                        ApiResult::Failure{msg:"Failed to find money map. Invalid user ID."}
                                    }
                                }
                            },
                            Err(e) => {
                                error!("{}", e);
                                ApiResult::Failure{msg:"Failed to find money map. Invalid ID."}
                            }
                        }

                        // START Retrieve DAO Error Handling --------------------------------------
                    },
                    Err(e) => {
                        error!("{}",e.get_message().to_string());
                        ApiResult::Failure{msg:"Unable to interact with database"}
                    }
                }
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
        // END Retrieve DAO Error Handling --------------------------------------------------------
    }// end create

    /// Retrieve all money map accounts
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    /// mm_id - String Money Map ID
    ///
    /// # Returns
    /// `ApiResult<Vec<MoneyMapModel>>` - ApiResult including a vector of money maps
    pub fn find(&self, req: &mut Request<ServerData>, mm_id: String) -> ApiResult<Vec<OutAccountModel>, ()>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        // START Retrieve DAO ---------------------------------------------------------------------
        match self.dao_manager.get_account_dao(){
            Ok(account_dao) => {
                // END Retrieve DAO ---------------------------------------------------------------

                match ObjectId::with_string(&mm_id){
                    Ok(mm_obj_id) => {
                        match ObjectId::with_string(&user_id){
                            Ok(user_obj_id) => {

                                // Get list of accounts for money map
                                match account_dao.find(user_obj_id, mm_obj_id){
                                    Some(accounts) => {
                                        ApiResult::Success{
                                            result:accounts.into_iter().map(|x| OutAccountModel::new(x)).collect()
                                        }
                                    },
                                    None => {
                                        ApiResult::Failure{msg:"Unable to find money map."}
                                    }
                                }
                            },
                            Err(e) => {
                                error!("{}", e);
                                ApiResult::Failure{msg:"Failed to find money map. Invalid user ID."}
                            }
                        }
                    },
                    Err(e) => {
                        error!("{}", e);
                        ApiResult::Failure{msg:"Failed to find money map. Invalid ID."}
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

    /// Modify Account
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    /// mm_id - String Money Map ID
    ///
    /// # Returns
    /// `ApiResult<AccountModel, AccountModel>` - ApiResult including the modified Account
    pub fn modify(&self, req: &mut Request<ServerData>, mm_id: String) -> ApiResult<OutAccountModel, AccountModel>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        // START Retrieve DAO ---------------------------------------------------------------------
        match self.dao_manager.get_account_dao(){
            Ok(account_dao) => {
                // END Retrieve DAO ---------------------------------------------------------------

                match ObjectId::with_string(&user_id){
                    Ok(user_obj_id) => {
                        match ObjectId::with_string(&mm_id){
                            Ok(mm_obj_id) => {

                                // Parse body to AccountModel
                                match req.json_as::<AccountModel>(){
                                    Ok(account) => {

                                        //Validate
                                        let validation_result = account.validate_existing();
                                        if validation_result.is_valid(){
                                            //Save the account
                                            match account_dao.update(mm_obj_id, user_obj_id, &account){
                                                Ok(updated_account) => {
                                                    ApiResult::Success{result:OutAccountModel::new(updated_account)}
                                                },
                                                Err(e) => {
                                                    ApiResult::Failure{msg:e.get_message()}
                                                }
                                            }
                                        }else{
                                            ApiResult::Invalid{validation:validation_result, request:account}
                                        }

                                    },
                                    Err(e) => {
                                        error!("{}",e);
                                        ApiResult::Failure{msg:"Invalid format. Unable to parse data."}
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
    }// end modify

    /// Delete an Account
    ///
    /// # Arguments
    /// &self
    /// req - &nickel::Request
    /// mm_id - String Money Map ID
    /// acc_id - String Account ID
    ///
    /// # Returns
    /// `ApiResult<String>` - ApiResult
    pub fn delete(&self, req: &Request<ServerData>, mm_id: String, acc_id: String) -> ApiResult<String, ()>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        match self.dao_manager.get_account_dao(){
            Ok(dao) => {

                match ObjectId::with_string(&user_id){
                    Ok(user_obj_id) => {
                        match ObjectId::with_string(&mm_id){
                            Ok(mm_obj_id) => {
                                match ObjectId::with_string(&acc_id){
                                    Ok(acc_obj_id) => {

                                        match dao.delete(user_obj_id, mm_obj_id, acc_obj_id){
                                            Ok(result) => {
                                                if result.modified_count > 0 {
                                                    ApiResult::Success{result:"Successfully deleted account".to_string()}
                                                }else if result.acknowledged && result.matched_count == 0{
                                                    ApiResult::Failure{msg:"Unable to find account"}
                                                }else{
                                                    ApiResult::Failure{msg:"Unable to delete account"}
                                                }
                                            },
                                            Err(e) => {
                                                error!("{}",e);
                                                ApiResult::Failure{msg:"Malformed ID"}
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        error!("{}", e);
                                        ApiResult::Failure{msg:"Failed to delete account. Invalid account ID."}
                                    }
                                }
                            },
                            Err(e) => {
                                error!("{}", e);
                                ApiResult::Failure{msg:"Failed to delete account. Invalid money map ID."}
                            }
                        }
                    },
                    Err(e) => {
                        error!("{}", e);
                        ApiResult::Failure{msg:"Failed to delete account. Invalid user ID."}
                    }
                }
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
    }// end delete

}
