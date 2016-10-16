// src/controllers/money_map_users_controller.rs

/// Money Map Users Controller

// Import
// External
use ::nickel::{JsonBody, Request};
use ::bson::{Bson};
use ::bson::oid::ObjectId;
// Utilities
use ::common::api_result::ApiResult;
use ::common::config::Config;
use ::common::data_access::ServerData;
use ::common::mm_result::{MMResult, MMError, MMErrorKind};
use ::common::session as Session;
// DAO
use ::dao::dao_manager::DAOManager;
// Models
use ::models::user_model::{OutUserModel};
use ::models::money_map_model::{MoneyMapModel};
use ::models::money_map_user_model::{MoneyMapUserModel, InMoneyMapUserModel};

#[derive(Clone)]
pub struct MoneyMapUsersController{
    dao_manager: DAOManager,
    config: Config
}

impl MoneyMapUsersController{

    pub fn new(dao_manager: DAOManager, config: Config) -> MoneyMapUsersController{
        MoneyMapUsersController{
            dao_manager: dao_manager,
            config: config
        }
    }

    /// Retrieve list of users for a money map with following conditions
    ///     Money map information only available for money maps belonging to current user
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<Vec<MoneyMapUserModel>>` - ApiResult including the list of money map users
    pub fn find(&self, req: &Request<ServerData>, mm_id: &str) -> ApiResult<Vec<MoneyMapUserModel>, ()>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        match self.dao_manager.get_money_map_dao(){
            Ok(dao) => {

                match ObjectId::with_string(mm_id){
                    Ok(id) => {
                        match ObjectId::with_string(&user_id){
                            Ok(user_obj_id) => {
                                //Get list of money maps for this user
                                let filter = doc!{
                                    "_id" => id,
                                    "users.user_id" => user_obj_id,
                                    "deleted" => {
                                        "$ne" => true
                                    }
                                };
                                match dao.find_one(Some(filter), None){
                                    Some(mut money_map) => {

                                        // Get list of user details for money map
                                        match MoneyMapUsersController::get_users_for_mm(&self.dao_manager, &money_map){
                                            Ok(users_list) => {
                                                // Add the new list of user details to the money map
                                                money_map.set_users(Some(users_list));
                                            },
                                            Err(e) => {
                                                return ApiResult::Failure{msg:e.get_message()};
                                            }
                                        }

                                        // Return the list of money maps
                                        match money_map.get_users(){
                                            Some(users) => ApiResult::Success{result:users},
                                            None => ApiResult::Failure{msg:"Unable to find user details for money map"}
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
            },
            Err(e) => {
                error!("{}",e.get_message().to_string());
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
    }// end find

    /// Add User to Money Map with following conditions
    ///     Users can only be added to a money map by the money map owner
    ///
    /// # Arguments
    /// &self
    /// req - nickel::Request
    ///
    /// # Returns
    /// `ApiResult<String, InMoneyMapUserModel>`
    pub fn add(&self, req: &mut Request<ServerData>, mm_id: String) -> ApiResult<String, InMoneyMapUserModel>{

        let user_id = match Session::get_session_id(req){
            Ok(id) => id,
            Err(e) => {
                error!("{}",e.get_message().to_string());
                return ApiResult::Failure{msg:"Unable to retrieve session data."};
            }
        };

        // START Retrieve DAO ---------------------------------------------------------------------
        match self.dao_manager.get_money_map_user_dao(){
            Ok(mm_user_dao) => {
                match self.dao_manager.get_money_map_dao(){
                    Ok(mm_dao) => {
                        match self.dao_manager.get_user_dao(){
                            Ok(user_dao) => {
                                // END Retrieve DAO -----------------------------------------------

                                match req.json_as::<InMoneyMapUserModel>(){
                                    Ok(mut money_map_user) => {

                                        match ObjectId::with_string(&mm_id){
                                            Ok(id) => {
                                                match ObjectId::with_string(&user_id){
                                                    Ok(user_obj_id) => {
                                                        //Find Money Map
                                                        let filter = doc!{
                                                            "_id" => id,
                                                            "users.user_id" => user_obj_id,
                                                            "deleted" => {
                                                                "$ne" => true
                                                            }
                                                        };
                                                        match mm_dao.find_one(Some(filter), None){
                                                            Some(mut money_map) => {

                                                                // Standard Validation
                                                                let mut validation_result = money_map_user.validate();
                                                                if validation_result.is_valid(){

                                                                    // Get user for email
                                                                    let filter = doc!{
                                                                        "email" => (money_map_user.clone().get_email().unwrap())
                                                                    };
                                                                    if let Some(user) = user_dao.find_one(Some(filter), None){

                                                                        // A user has been found with this email address, verify the user isn't already a member of this money map
                                                                        let user_id = user.get_id().unwrap();
                                                                        if let Some(mm_users) = money_map.get_users(){
                                                                            for mm_user in mm_users{
                                                                                if mm_user.get_user().unwrap().get_id().unwrap() == user_id {
                                                                                    // User already member of money map validation
                                                                                    validation_result.add_error("email".to_string(), "User already a member of this money map".to_string());
                                                                                    return ApiResult::Invalid{validation:validation_result, request:money_map_user};
                                                                                }
                                                                            }
                                                                        }

                                                                        // All is well, add the user to the money map
                                                                        if let Ok(result) = mm_user_dao.add_user(money_map.get_id().unwrap(), user.get_id().unwrap()){
                                                                            if result.acknowledged && result.modified_count > 0 {
                                                                                ApiResult::Success{result:"Successfully added user to money map".to_string()}
                                                                            }else{
                                                                                ApiResult::Failure{msg:"Unable to add user to money map"}
                                                                            }
                                                                        }else{
                                                                            ApiResult::Failure{msg:"Error adding user to money map"}
                                                                        }

                                                                    }else{
                                                                        // User not found validation
                                                                        validation_result.add_error("email".to_string(), "A user cannot be found with this email address.".to_string());
                                                                        ApiResult::Invalid{validation:validation_result, request:money_map_user}
                                                                    }
                                                                }else{
                                                                    // Standard Validation failed
                                                                    ApiResult::Invalid{validation:validation_result, request:money_map_user}
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
                                    },
                                    Err(e) => {
                                        error!("{}",e);
                                        ApiResult::Failure{msg:"Invalid format. Unable to parse data."}
                                    }
                                }


                                // START Retrieve DAO Error Handling ------------------------------
                            },
                            Err(e) => {
                                // Unable to retrieve UserDAO
                                error!("{}",e.get_message().to_string());
                                ApiResult::Failure{msg:"Unable to interact with database"}
                            }
                        }
                    },
                    Err(e) => {
                        // Unable to retrieve MoneyMapDAO
                        error!("{}",e.get_message().to_string());
                        ApiResult::Failure{msg:"Unable to interact with database"}
                    }
                }
            },
            Err(e) => {
                // Unable to retrieve MoneyMapUserDAO
                error!("{}",e.get_message().to_string());
                ApiResult::Failure{msg:"Unable to interact with database"}
            }
        }
        // END Retrieve DAO Error Handling --------------------------------------------------------

    }// end add

    /// Delete a Money Map User
    /// Remove a user from money map using the following rules:
    ///     Only an owner can remove users from a money map.
    ///     Owner should not be allowed to be deleted.
    ///
    /// # Arguments
    /// &self
    /// id - String
    ///
    /// # Returns
    /// `ApiResult<Vec<MoneyMapUserModel>>` - ApiResult including the updated list of money map users
    pub fn delete(&self, mm_id: &str, user_id: &str) -> ApiResult<Vec<MoneyMapUserModel>, ()>{
        ApiResult::Failure{msg:"Delete needs to be implemented"}
    }// end delete

    /// Get a list of user details for money map
    ///
    /// # Arguments
    /// &self
    /// money_map - &MoneyMapModel
    ///
    /// # Returns
    /// `ApiResult<Vec<MoneyMapUserModel>>` - MMResult including the list of money map users
    pub fn get_users_for_mm(dao_manager: &DAOManager, money_map: &MoneyMapModel) -> MMResult<Vec<MoneyMapUserModel>>{
        // Initialze a list of user details for this money map
        let mut users_list = Vec::new();
        if let Some(mm_users) = money_map.get_users(){

            // For each user associated with this money map
            for mm_user in mm_users{
                match dao_manager.get_user_dao(){
                    Ok(user_dao) => {

                        // Fetch the user's details
                        let user_id = mm_user.user.unwrap().id.unwrap();
                        let found_user = user_dao.find_one(Some(doc!{
                            "_id" => user_id
                        }), None);
                        if let Some(user) = found_user{
                            // Add the user details to the list
                            users_list.push(
                                MoneyMapUserModel::new(OutUserModel::new(user), mm_user.owner)
                            );
                        }
                    },
                    Err(e) => {
                        error!("{}",e.get_message().to_string());
                        return Err(MMError::new("Unable to interact with database", MMErrorKind::Controller));
                    }
                }
            }
        }
        Ok(users_list)
    }// end get_users_for_mm
}
